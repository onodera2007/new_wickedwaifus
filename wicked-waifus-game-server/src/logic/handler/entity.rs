use wicked_waifus_protocol::{
    EntityAccessInfo, EntityAccessRangeRequest, EntityAccessRangeResponse, EntityActiveRequest,
    EntityActiveResponse, EntityFollowTrackRequest, EntityFollowTrackResponse,
    EntityInteractRequest, EntityInteractResponse, EntityOnLandedRequest, EntityOnLandedResponse,
    EntityPb, EntityPositionRequest, EntityPositionResponse, ErrorCode,
    GetRewardTreasureBoxRequest, GetRewardTreasureBoxResponse, MovePackagePush,
};

use wicked_waifus_data::pb_components::option::OptionType;

use crate::logic::thread_mgr::NetContext;
use crate::logic::utils::action_utils::perform_action;
use crate::logic::utils::condition_utils::check_condition;
use crate::{logic, logic::ecs::component::ComponentContainer, query_components};

pub fn on_entity_active_request(
    ctx: &NetContext,
    request: EntityActiveRequest,
    response: &mut EntityActiveResponse,
) {
    let world = ctx.world.get_world_entity();

    if !world.is_in_all_world_map(request.entity_id as i32) {
        tracing::debug!(
            "EntityActiveRequest: entity with id {} doesn't exist, player_id: {}",
            request.entity_id,
            ctx.player.basic_info.id
        );
        return;
    };

    let component_pbs = {
        let mut pb = EntityPb {
            id: request.entity_id,
            ..Default::default()
        };

        world
            .get_entity_components(request.entity_id as i32)
            .into_iter()
            .for_each(|comp| comp.set_pb_data(&mut pb));
        pb.component_pbs
    };

    // TODO: Remove attribute
    if let (Some(position), Some(_attribute), Some(visibility)) =
        query_components!(world, request.entity_id, Position, Attribute, Visibility)
    {
        response.is_visible = visibility.is_visible;
        
        response.pos = Some(position.0.get_position_protobuf());
        response.rot = Some(position.0.get_rotation_protobuf());

        response.component_pbs.extend_from_slice(&component_pbs);

        response.error_code = ErrorCode::Success.into();
    } else {
        tracing::error!(
            "EntityActiveRequest: entity with id {} not found",
            request.entity_id
        );
        response.error_code = ErrorCode::ErrEntityNotFound.into(); // TODO: replace with appropriate error code;
    };
}

pub fn on_entity_on_landed_request(
    _: &NetContext,
    request: EntityOnLandedRequest,
    _: &mut EntityOnLandedResponse,
) {
    // TODO: More implementation?
    tracing::debug!(
        "EntityOnLandedRequest: entity with id {} landed",
        request.entity_id
    );
}

pub fn on_entity_position_request(
    _: &NetContext,
    request: EntityPositionRequest,
    _: &mut EntityPositionResponse,
) {
    // TODO: Implement this
    tracing::debug!(
        "EntityPositionRequest: config with id {} for map {} position requested",
        request.config_id,
        request.dungeon_instance_id
    );
}

pub fn on_move_package_push(ctx: &mut NetContext, push: MovePackagePush) {
    for moving_entity in push.moving_entities {
        {
            let world = ctx.world.get_world_entity();

            if !world.is_in_all_world_map(moving_entity.entity_id as i32) {
                tracing::debug!(
                    "MovePackage: entity with id {} doesn't exist",
                    moving_entity.entity_id
                );
                continue;
            }

            let Some(mut movement) = query_components!(world, moving_entity.entity_id, Movement).0
            else {
                tracing::warn!(
                    "MovePackage: entity {} doesn't have movement component",
                    moving_entity.entity_id
                );
                continue;
            };

            movement
                .pending_movement_vec
                .extend(moving_entity.move_infos);
        }

        let map = logic::utils::quadrant_util::get_map(ctx.player.basic_info.cur_map_id);
        let quadrant_id = map.get_quadrant_id(
            ctx.player.location.position.position.x * 100.0,
            ctx.player.location.position.position.y * 100.0,
        );

        // TODO: This may require some changes for Co-Op
        if quadrant_id != ctx.player.quadrant_id {
            let (entities_to_remove, entities_to_add) =
                map.get_update_entities(ctx.player.quadrant_id, quadrant_id);
            ctx.player.quadrant_id = quadrant_id;
            logic::utils::world_util::remove_entities(ctx, &entities_to_remove);
            logic::utils::world_util::add_entities(ctx, &entities_to_add, false);
        }
    }
}

pub fn on_entity_access_range_request(
    _: &NetContext,
    request: EntityAccessRangeRequest,
    response: &mut EntityAccessRangeResponse,
) {
    // TODO: from world fetch entity by request.entity_id
    // TODO: Compute the distance between player and entity.entity_id
    response.error_code = ErrorCode::Success.into();
    response.entity_id = request.entity_id;
    let mut infos = Vec::new();
    for range_request in request.entities_to_check {
        infos.push(EntityAccessInfo {
            entity_id: range_request,
            range_type: request.range_type,
            uo_1: Default::default(),
        })
    }
    response.info = infos;
}

pub fn on_entity_interact_request(
    ctx: &mut NetContext,
    request: EntityInteractRequest,
    response: &mut EntityInteractResponse,
) {
    let config_id = get_config_id_from_entity_id(ctx, request.entity_id);
    tracing::debug!(
        "EntityInteractRequest with ID: {} and ConfigID {}",
        request.entity_id,
        config_id
    );

    // TODO: add cases outside LevelEntityConfig if exist
    let Some(entity) = wicked_waifus_data::level_entity_config_data::get(
        ctx.player.basic_info.cur_map_id,
        config_id,
    ) else {
        response.error_code = ErrorCode::ErrEntityNotFound.into();
        return;
    };

    let Some(template_config) =
        wicked_waifus_data::template_config_data::get(&entity.blueprint_type)
    else {
        response.error_code = ErrorCode::ErrEntityNotFound.into();
        return;
    };

    let Some(interact_component) = entity
        .components_data
        .interact_component
        .as_ref()
        .or(template_config.components_data.interact_component.as_ref())
        .cloned()
    else {
        response.error_code = ErrorCode::ErrInteractComponentNotExist.into();
        return;
    };

    let Some(options) = interact_component.options else {
        response.error_code = ErrorCode::ErrActionNoInteractConfig.into();
        return;
    };
    for option in options {
        let mut check = true;
        if let Some(conditions) = option.condition {
            for element in conditions.conditions {
                check = check_condition(ctx, request.entity_id, entity, template_config, element);
                if !check {
                    break;
                }
            }
        }
        if check {
            if let Some(option_type) = option.r#type {
                match option_type {
                    OptionType::Actions(actions) => {
                        for action in actions.actions {
                            perform_action(
                                ctx,
                                request.entity_id,
                                entity,
                                template_config,
                                action,
                            );
                        }
                    }
                    OptionType::Flow(_) => {
                        tracing::warn!("Option Type: Flow not implemented");
                    }
                };
            }
        }
    }

    response.error_code = ErrorCode::Success.into();
    response.interacting = true;
}

pub fn on_entity_follow_track_request(
    ctx: &mut NetContext,
    request: EntityFollowTrackRequest,
    response: &mut EntityFollowTrackResponse,
) {
    let config_id = get_config_id_from_entity_id(ctx, request.entity_id);
    let position = {
        let world = ctx.world.get_world_entity();
        let position = query_components!(world, request.entity_id, Position)
            .0
            .unwrap();
        position.0.clone()
    };
    tracing::debug!(
        "EntityFollowTrackRequest with ID: {} and ConfigID {config_id} and position {position:?}",
        request.entity_id
    );

    response.error_code = ErrorCode::Success.into();
}

pub fn on_get_reward_treasure_box_request(
    ctx: &NetContext,
    request: GetRewardTreasureBoxRequest,
    _response: &mut GetRewardTreasureBoxResponse,
) {
    let config_id = get_config_id_from_entity_id(ctx, request.entity_id);
    tracing::debug!(
        "GetRewardTreasureBoxRequest with ID: {} and ConfigID {config_id}",
        request.entity_id
    );
}

fn get_config_id_from_entity_id(ctx: &NetContext, entity_id: i64) -> i64 {
    let world = ctx.world.get_world_entity();
    let entity_config = query_components!(world, entity_id, EntityConfig).0.unwrap();
    entity_config.config_id as i64
}
