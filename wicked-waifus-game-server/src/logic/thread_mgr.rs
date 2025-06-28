use wicked_waifus_commons::time_util;
use wicked_waifus_protocol_internal::PlayerSaveData;
use wicked_waifus_protocol::{message::Message, AfterJoinSceneNotify, EnterGameResponse, JoinSceneNotify, SilenceNpcNotify, TransitionOptionPb};
use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    rc::Rc,
    sync::{
        atomic::{AtomicUsize, Ordering},
        mpsc, Arc, OnceLock,
    },
    thread,
    time::Duration,
};
use super::{ecs::world::World, player::Player, utils::world_util};
use crate::logic::ecs::world::WorldEntity;
use crate::{logic, player_save_task::{self, PlayerSaveReason}, session::Session};

pub enum LogicInput {
    AddPlayer {
        player_id: i32,
        enter_rpc_id: u16,
        session: Arc<Session>,
        player_save_data: PlayerSaveData,
    },
    RemovePlayer {
        player_id: i32,
    },
    ProcessMessage {
        player_id: i32,
        message: Message,
    },
}

#[derive(Clone)]
pub struct LogicThreadHandle {
    sender: mpsc::Sender<LogicInput>,
    load: Arc<AtomicUsize>,
}

static THREAD_HANDLES: OnceLock<Box<[LogicThreadHandle]>> = OnceLock::new();

pub fn start_logic_threads(num_threads: usize) {
    if THREAD_HANDLES.get().is_some() {
        tracing::error!("start_logic_threads: logic threads are already running!");
        return;
    }

    let _ = THREAD_HANDLES.set(
        (0..num_threads)
            .map(|_| {
                let (tx, rx) = mpsc::channel();
                let load = Arc::new(AtomicUsize::new(0));

                let handle = LogicThreadHandle {
                    sender: tx,
                    load: load.clone(),
                };

                thread::spawn(move || logic_thread_func(rx, load));
                handle
            })
            .collect(),
    );
}

// Thread-local logic state
struct LogicState {
    thread_load: Arc<AtomicUsize>, // shared parameter for load-balancing
    worlds: HashMap<i32, Rc<RefCell<World>>>, // owner_id - world
    players: HashMap<i32, RefCell<Player>>, // id - player
}

fn logic_thread_func(receiver: mpsc::Receiver<LogicInput>, load: Arc<AtomicUsize>) {
    const RECV_TIMEOUT: Duration = Duration::from_millis(50);
    const PLAYER_SAVE_PERIOD: u64 = 30;

    let mut state = LogicState {
        thread_load: load,
        worlds: HashMap::new(),
        players: HashMap::new(),
    };

    let mut input_queue = VecDeque::with_capacity(32);

    loop {
        if let Ok(input) = receiver.recv_timeout(RECV_TIMEOUT) {
            input_queue.push_back(input);

            while let Ok(input) = receiver.try_recv() {
                input_queue.push_back(input);
            }
        }

        while let Some(input) = input_queue.pop_front() {
            handle_logic_input(&mut state, input);
        }

        state.worlds.values().for_each(|world| {
            let mut world = world.borrow_mut();
            let mut players = world
                .player_ids()
                .flat_map(|id| state.players.get(id).map(|pl| pl.borrow_mut()))
                .collect::<Box<_>>();

            super::systems::tick_systems(&mut world, &mut players);
        });

        state.players.values().for_each(|player| {
            let mut player = player.borrow_mut();
            if time_util::unix_timestamp() - player.last_save_time > PLAYER_SAVE_PERIOD {
                player_save_task::push(
                    player.basic_info.id,
                    player.build_save_data(),
                    PlayerSaveReason::PeriodicalSave,
                );

                player.last_save_time = time_util::unix_timestamp();
            }
        })
    }
}

pub struct NetContext<'logic> {
    pub player: &'logic mut Player,
    pub world: &'logic mut World,
}

fn handle_logic_input(state: &mut LogicState, input: LogicInput) {
    match input {
        LogicInput::AddPlayer {
            player_id,
            enter_rpc_id,
            session,
            player_save_data,
        } => {
            let mut player = state.players.entry(player_id).or_insert_with(|| {
                RefCell::new(Player::load_from_save(player_save_data))
            }).borrow_mut();

            // TODO: shall we search in coop?
            player.world_owner_id = player_id;
            let mut world = state.worlds.entry(player_id).or_insert_with(|| {
                let mut world = World::new();
                world.world_entities.insert(
                    player.basic_info.cur_map_id,
                    WorldEntity::default(),
                );
                Rc::new(RefCell::new(world))
            }).borrow_mut();

            player.init();
            player.set_session(session);
            player.respond(EnterGameResponse::default(), enter_rpc_id);
            player.notify_general_data();

            world.set_in_world_player_data(player.build_in_world_player());

            let mut ctx = NetContext {
                player: &mut player,
                world: &mut world,
            };
            world_util::add_player_entities(&mut ctx);
            let scene_info = world_util::build_scene_information(&mut ctx);

            ctx.player.notify(SilenceNpcNotify::default());

            ctx.player.notify(JoinSceneNotify {
                scene_info: Some(scene_info),
                max_entity_id: i64::MAX,
                transition_option: Some(TransitionOptionPb::default()),
            });

            ctx.player.notify(AfterJoinSceneNotify::default());
            ctx.player.notify(ctx.player.build_update_formation_notify());

            let map = logic::utils::quadrant_util::get_map(ctx.player.basic_info.cur_map_id);
            let quadrant_id = map.get_quadrant_id(
                ctx.player.location.position.position.x * 100.0,
                ctx.player.location.position.position.y * 100.0,
            );
            ctx.player.quadrant_id = quadrant_id;
            ctx.player.notify_month_card();

            let entities = map.get_initial_entities(quadrant_id);
            world_util::add_entities(&mut ctx, &entities, false);

            drop(player);

            state
                .thread_load
                .store(state.players.len(), Ordering::Relaxed);
        }
        LogicInput::ProcessMessage { player_id, message } => {
            let Some(player) = state.players.get_mut(&player_id) else {
                tracing::warn!("logic_thread: process message requested, but player with id {player_id} doesn't exist");
                return;
            };
            let mut player = player.borrow_mut();
            let Some(world) = state.worlds.get_mut(&player.world_owner_id) else {
                tracing::warn!("logic_thread: process message requested, but world for player id {} doesn't exist", player.world_owner_id);
                return;
            };
            let mut world = world.borrow_mut();
            let mut net_context = NetContext {
                player: &mut player,
                world: &mut world,
            };
            super::handler::handle_logic_message(&mut net_context, message);
        }
        LogicInput::RemovePlayer { player_id } => {
            let Some(player) = state.players.remove(&player_id) else {
                tracing::warn!(
                    "logic_thread: player remove requested, but it doesn't exist (id: {player_id})"
                );
                return;
            };

            let _ = state.worlds.remove(&player_id);
            // TODO: kick co-op players from removed world
            // TODO: Remove all entities

            player_save_task::push(
                player_id,
                player.borrow().build_save_data(),
                PlayerSaveReason::PlayerLogicStopped,
            );
        }
    }
}

impl LogicThreadHandle {
    pub fn input(&self, input: LogicInput) {
        let _ = self.sender.send(input);
    }
}

pub fn get_least_loaded_thread() -> LogicThreadHandle {
    let handles = THREAD_HANDLES.get().unwrap();
    handles
        .iter()
        .min_by_key(|h| h.load.load(Ordering::Relaxed))
        .unwrap()
        .clone()
}
