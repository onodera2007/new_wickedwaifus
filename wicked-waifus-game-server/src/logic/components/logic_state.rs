use wicked_waifus_protocol::entity_component_pb::ComponentPb;
use wicked_waifus_protocol::{EntityComponentPb, LogicStateComponentPb};

use crate::logic::ecs::component::Component;

pub struct LogicState {
    pub position_state: i32,
    pub move_state: i32,
    pub direction_state: i32,
    pub position_sub_state: i32,
}

impl Component for LogicState {
    fn set_pb_data(&self, pb: &mut wicked_waifus_protocol::EntityPb) {
        pb.component_pbs.push(EntityComponentPb {
            component_pb: Some(ComponentPb::LogicStateComponentPb(LogicStateComponentPb {
                position_state: self.position_state,
                move_state: self.move_state,
                direction_state: self.direction_state,
                position_sub_state: self.position_sub_state,
            })),
        })
    }
}
