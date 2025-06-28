use crate::logic::thread_mgr::NetContext;
use wicked_waifus_protocol::{
    ItemExchangeInfo, ItemExchangeInfoRequest, ItemExchangeInfoResponse, NormalItemRequest,
    NormalItemResponse, PhantomItemRequest, PhantomItemResponse, WeaponItemRequest,
    WeaponItemResponse,
};

pub fn on_normal_item_request(
    ctx: &NetContext,
    _: NormalItemRequest,
    response: &mut NormalItemResponse,
) {
    tracing::debug!("Received NormalItemRequest, returning player inventory");
    response.normal_item_list = ctx.player.inventory.to_normal_item_list();
}

pub fn on_weapon_item_request(
    ctx: &NetContext,
    _: WeaponItemRequest,
    response: &mut WeaponItemResponse,
) {
    response.weapon_item_list = ctx.player.inventory.to_weapon_item_list();
}

pub fn on_phantom_item_request(
    ctx: &mut NetContext,
    _: PhantomItemRequest,
    response: &mut PhantomItemResponse,
) {
    let (items, equip_info, prop_info) = ctx.player.inventory.get_echoes_list();
    response.phantom_item_list = items;
    response.equip_info_list = equip_info;
    response.ows = prop_info;
    response.max_cost = 8; // TODO: Max cost from calabash
    response.phantom_skin_list = ctx
        .player
        .unlocked_skins
        .echo_skins
        .iter()
        .copied()
        .collect();
}

pub fn on_item_exchange_info_request(
    _ctx: &mut NetContext,
    _: ItemExchangeInfoRequest,
    response: &mut ItemExchangeInfoResponse,
) {
    response.item_exchange_infos = wicked_waifus_data::item_exchange_content_data::iter()
        .map(|item_exchange_content_data| ItemExchangeInfo {
            item_id: item_exchange_content_data.item_id,
            today_times: 0, // TODO: For stats only, not used for PS so far
            total_times: 0, // TODO: For stats only, not used for PS so far
            daily_limit: 0, // At the time of writing there is no limits
            total_limit: 0, // At the time of writing there is no limits
        })
        .collect();
}
