use crate::logic::ecs::component::ComponentContainer;
use wicked_waifus_protocol::{EEntityType, EntityPb, PlayerSceneAoiData};
use std::collections::HashSet;

use crate::logic::components::Visibility;
use crate::{modify_component, query_hn_with};
use crate::logic::thread_mgr::NetContext;

pub fn build_scene_add_on_init_data(ctx: &mut NetContext) -> PlayerSceneAoiData {
    let world = ctx.world.get_mut_world_entity();
    let entities = query_hn_with!(world, PlayerOwnedEntityMarker)
        .into_iter()
        .map(|(entity_id, _)| {
            match EEntityType::try_from(
                world.get_entity(world.get_config_id(entity_id)).entity_type,
            ) {
                Ok(EEntityType::Player) => {
                    (EEntityType::Player, entity_id)
                }
                Ok(EEntityType::Monster) => {
                    (EEntityType::Monster, entity_id)
                }
                _ => {
                    (EEntityType::default(), -1)
                }
            }
        })
        .collect::<HashSet<(EEntityType, i32)>>();

    let mut aoi_data = PlayerSceneAoiData::default();

    entities
        .iter()
        .filter(|&&(_, entity_id)| entity_id != -1)
        .for_each(|&(entity_type, entity_id)| {
            match entity_type {
                EEntityType::Player => {
                    let config_id = world.get_config_id(entity_id);
                    modify_component!(
                        world.get_entity_components(entity_id),
                        Visibility,
                        |vis: &mut Visibility| {
                            let cur_role_id = ctx.player
                                .formation_list
                                .get(&ctx.player.cur_formation_id)
                                .unwrap()
                                .cur_role;
                            (vis.is_visible, vis.is_actor_visible) = if config_id == cur_role_id {
                                (true, true)
                            } else {
                                (false, true)
                            };
                        }
                    );

                    if world.get_entity(config_id).entity_type == EEntityType::Player as i32 {
                        let mut pb = EntityPb {
                            id: entity_id as i64,
                            ..Default::default()
                        };
                        world
                            .get_entity_components(entity_id)
                            .into_iter()
                            .for_each(|comp| comp.set_pb_data(&mut pb));

                        aoi_data.entities.push(pb);
                    }
                }
                EEntityType::Monster => {
                    let config_id = world.get_config_id(entity_id);
                    modify_component!(
                        world.get_entity_components(entity_id),
                        Visibility,
                        |vis: &mut Visibility| {
                            vis.is_visible = false;
                            vis.is_actor_visible = true;
                        }
                    );
                    if world.get_entity(config_id).entity_type == EEntityType::Monster as i32 {
                        let mut pb = EntityPb {
                            id: entity_id as i64,
                            ..Default::default()
                        };
                        world
                            .get_entity_components(entity_id)
                            .into_iter()
                            .for_each(|comp| comp.set_pb_data(&mut pb));
                        aoi_data.entities.push(pb);
                    }
                }
                _ => {}
            };
        });

    aoi_data
}
