use wicked_waifus_protocol::{
    ErrorCode, InputSettingRequest, InputSettingResponse, InputSettingUpdateRequest,
    InputSettingUpdateResponse, LanguageSettingUpdateRequest, LanguageSettingUpdateResponse,
    MonthCardRequest, MonthCardResponse, ServerPlayStationPlayOnlyStateRequest,
    ServerPlayStationPlayOnlyStateResponse, UpdateVoxelEnvRequest, UpdateVoxelEnvResponse,
    VersionInfoPush, WebSignRequest, WebSignResponse, Zih,
};

use crate::logic::thread_mgr::NetContext;

pub fn on_month_card_request(
    ctx: &mut NetContext,
    _: MonthCardRequest,
    response: &mut MonthCardResponse,
) {
    // TODO: Check if we should send MonthCardUseNotify
    response.days = ctx.player.month_card.days;
    response.is_daily_got =
        wicked_waifus_commons::time_util::unix_days() == ctx.player.month_card.last_received_day;
    response.error_code = ErrorCode::Success.into();
}

pub fn on_web_sign_request(_: &mut NetContext, _: WebSignRequest, response: &mut WebSignResponse) {
    response.notice_sign =
        "Welcome to Wicked Waifus PS provided by Reversed Rooms Dev Team".to_string();
}

pub fn on_input_setting_request(
    _: &NetContext,
    _: InputSettingRequest,
    response: &mut InputSettingResponse,
) {
    response.zih = Some(Zih::default());
}

pub fn on_input_setting_update_request(
    _: &NetContext,
    _: InputSettingUpdateRequest,
    response: &mut InputSettingUpdateResponse,
) {
    response.error_code = ErrorCode::Success.into();
}

pub fn on_language_setting_update_request(
    _: &NetContext,
    _: LanguageSettingUpdateRequest,
    response: &mut LanguageSettingUpdateResponse,
) {
    response.error_code = ErrorCode::Success.into();
}

pub fn on_server_play_station_play_only_state_request(
    _: &NetContext,
    _: ServerPlayStationPlayOnlyStateRequest,
    response: &mut ServerPlayStationPlayOnlyStateResponse,
) {
    response.cross_play_enabled = false;
}

pub fn on_version_info_push(_player: &NetContext, push: VersionInfoPush) {
    // TODO: Shall we do safety check and ensure we have compatible versions?
    tracing::debug!(
        "Client versions: launcher: {}, app: {}, resources: {}",
        push.launcher_version,
        push.app_version,
        push.resource_version
    );
}

pub fn on_update_voxel_env_request(
    _: &NetContext,
    request: UpdateVoxelEnvRequest,
    response: &mut UpdateVoxelEnvResponse,
) {
    response.server_cave_mode = request.server_cave_mode;
    response.error_code = ErrorCode::Success.into();
}
