use crate::logic::components::{Equip, WeaponSkin};
use crate::logic::ecs::component::ComponentContainer;
use crate::logic::player::Player;
use crate::logic::thread_mgr::NetContext;
use crate::modify_component;
use wicked_waifus_protocol::{
    EntityEquipChangeNotify, EntityEquipSkinChangeNotify, EquipComponentPb, EquipTakeOnNotify,
    EquipTakeOnRequest, EquipTakeOnResponse, EquipWeaponSkinRequest, EquipWeaponSkinResponse,
    ErrorCode, LoadEquipData, SendEquipSkinRequest, SendEquipSkinResponse, WeaponSkinComponentPb,
    WeaponSkinDeleteNotify, WeaponSkinRequest, WeaponSkinResponse,
};

pub fn on_weapon_skin_request(
    ctx: &NetContext,
    _request: WeaponSkinRequest,
    response: &mut WeaponSkinResponse,
) {
    response.equip_list = get_player_weapons(ctx.player);
    response.error_code = ErrorCode::Success.into();
}

pub fn on_equip_weapon_skin_request(
    ctx: &mut NetContext,
    request: EquipWeaponSkinRequest,
    response: &mut EquipWeaponSkinResponse,
) {
    let Some(equip_data) = request.data else {
        return;
    };

    let role = ctx.player.role_list.get_mut(&equip_data.role_id);
    let Some(role) = role else {
        response.error_code = ErrorCode::NotValidRole.into();
        return;
    };

    // Verify Id exist in bindata
    let Some(skin_data) =
        wicked_waifus_data::weapon_skin_data::iter().find(|data| data.id == equip_data.skin_id)
    else {
        response.error_code = ErrorCode::WeaponSkinDataErr.into();
        return;
    };

    // Verify Skin is unlocked
    if !ctx
        .player
        .unlocked_skins
        .weapon_skins
        .contains(&skin_data.id)
    {
        response.error_code = ErrorCode::WeaponSkinUnLockErr.into();
        return;
    }

    role.weapon_skin_id = equip_data.skin_id;
    let world = ctx.world.get_world_entity();
    let entity_id = world.get_entity_id(equip_data.role_id);
    modify_component!(
        world.get_entity_components(entity_id as i32),
        WeaponSkin,
        |skin_component: &mut WeaponSkin| {
            skin_component.skin_id = equip_data.skin_id;
        }
    );
    ctx.player.notify(EntityEquipSkinChangeNotify {
        entity_id,
        weapon_skin_component_pb: Some(WeaponSkinComponentPb {
            weapon_skin_id: equip_data.skin_id,
        }),
    });

    // Is the all list needed or only the new one??
    response.data_list = get_player_weapons(ctx.player);
    response.error_code = ErrorCode::Success.into();
}

pub fn on_send_equip_skin_request(
    ctx: &mut NetContext,
    request: SendEquipSkinRequest,
    response: &mut SendEquipSkinResponse,
) {
    let role = ctx.player.role_list.get_mut(&request.role_id);
    let Some(role) = role else {
        response.error_code = ErrorCode::NotValidRole.into();
        return;
    };

    let old_skin_id = role.weapon_skin_id;
    role.weapon_skin_id = 0;
    let world = ctx.world.get_world_entity();
    let entity_id = world.get_entity_id(request.role_id);
    modify_component!(
        world.get_entity_components(entity_id as i32),
        WeaponSkin,
        |skin_component: &mut WeaponSkin| {
            skin_component.skin_id = 0;
        }
    );
    ctx.player.notify(EntityEquipSkinChangeNotify {
        entity_id,
        weapon_skin_component_pb: Some(WeaponSkinComponentPb { weapon_skin_id: 0 }),
    });
    ctx.player.notify(WeaponSkinDeleteNotify {
        role_id: request.role_id,
        skin_id: old_skin_id,
    });
    response.error_code = ErrorCode::Success.into();
}

pub fn on_equip_take_on_request(
    ctx: &mut NetContext,
    request: EquipTakeOnRequest,
    response: &mut EquipTakeOnResponse,
) {
    let Some(equip_data) = request.data else {
        return;
    };

    // TODO: Add sanity checks(add from another role, a.k.a.: switch from roles)
    ctx.player.notify(EquipTakeOnNotify {
        data_list: vec![equip_data],
    });

    let role = ctx.player.role_list.get_mut(&equip_data.role_id);
    let Some(role) = role else {
        response.error_code = ErrorCode::NotValidRole.into();
        return;
    };

    let Some((id, breach)) = ctx
        .player
        .inventory
        .get_weapon_equip_info(equip_data.equip_inc_id)
    else {
        response.error_code = ErrorCode::ErrItemNotFound.into();
        return;
    };
    role.equip_weapon = id;

    // TODO: Change attributes based on weapon (PbRolePropsNotify + buffs + CombatNotifyAttributeChangedNotify)

    // TODO: remove from old one if in scene in case of weapon switch
    let world = ctx.world.get_world_entity();
    let entity_id = world.get_entity_id(equip_data.role_id);
    modify_component!(
        world.get_entity_components(entity_id as i32),
        Equip,
        |equip_component: &mut Equip| {
            equip_component.weapon_id = id;
            equip_component.weapon_breach_level = breach;
        }
    );
    ctx.player.notify(EntityEquipChangeNotify {
        entity_id,
        equip_component: Some(EquipComponentPb {
            weapon_id: id,
            weapon_breach_level: breach,
        }),
    });
    // TODO: Should we return all of them??
    response.data_list = vec![equip_data];
    response.error_code = ErrorCode::Success.into();
}

#[inline(always)]
fn get_player_weapons(player: &Player) -> Vec<LoadEquipData> {
    player
        .role_list
        .values()
        .filter(|role| role.weapon_skin_id != 0)
        .map(|role| LoadEquipData {
            role_id: role.role_id,
            skin_id: role.weapon_skin_id,
        })
        .collect()
}
