use wicked_waifus_protocol::entity_component_pb::ComponentPb;
use wicked_waifus_protocol::{CharacterAttachComponentPb, CharacterAttachInfo, EntityComponentPb};

use crate::logic::ecs::component::Component;

pub struct CharacterAttach {
    pub pb_combine_part_info_list: Vec<CharacterAttachInfo>,
    pub pb_combine_target_server_id: i64,
}

impl Component for CharacterAttach {
    fn set_pb_data(&self, pb: &mut wicked_waifus_protocol::EntityPb) {
        pb.component_pbs.push(EntityComponentPb {
            component_pb: Some(ComponentPb::CharacterAttachComponentPb(
                CharacterAttachComponentPb {
                    pb_combine_part_info_list: self.pb_combine_part_info_list.clone(),
                    pb_combine_target_server_id: self.pb_combine_target_server_id,
                },
            )),
        })
    }
}
