use crate::logic::ecs::component::ComponentContainer;
use wicked_waifus_protocol::{
    ErrorCode, ExploreSkillRouletteSetRequest, ExploreSkillRouletteSetResponse,
    VisionExploreSkillSetRequest, VisionExploreSkillSetResponse, VisionSkillChangeNotify,
    VisionSkillInformation,
};

use crate::logic::thread_mgr::NetContext;
use crate::query_with;

pub fn on_vision_explore_skill_set_request(
    ctx: &mut NetContext,
    request: VisionExploreSkillSetRequest,
    response: &mut VisionExploreSkillSetResponse,
) {
    ctx.player.explore_tools.active_explore_skill = request.skill_id;

    for (entity, owner, mut vision_skill) in
        query_with!(ctx.world.get_world_entity(), OwnerPlayer, VisionSkill)
    {
        if owner.0 == ctx.player.basic_info.id {
            vision_skill.skill_id = request.skill_id;
            ctx.player.notify(VisionSkillChangeNotify {
                entity_id: entity.into(),
                vision_skill_infos: vec![VisionSkillInformation {
                    skill_id: request.skill_id,
                    ..Default::default()
                }],
                ..Default::default()
            })
        }
    }

    response.skill_id = request.skill_id;
}

pub fn on_explore_skill_roulette_set_request(
    ctx: &mut NetContext,
    request: ExploreSkillRouletteSetRequest,
    response: &mut ExploreSkillRouletteSetResponse,
) {
    let mut illegal_skill = false;
    for skill_roulette in &request.skill_roulettes {
        for skill_id in &skill_roulette.skill_ids {
            if *skill_id != 0
                && !ctx
                    .player
                    .explore_tools
                    .unlocked_explore_skills
                    .contains(skill_id)
            {
                illegal_skill = true;
                break;
            }
        }
    }
    match illegal_skill {
        true => response.error_code = ErrorCode::ErrRouletteFuncIdInvaild.into(),
        false => {
            ctx.player.explore_tools.roulette = request
                .skill_roulettes
                .first()
                .unwrap()
                .skill_ids
                .to_vec()
                .as_slice()
                .try_into()
                .unwrap();
            response.error_code = ErrorCode::Success.into();
            response.skill_roulettes = request.skill_roulettes;
        }
    }
}
