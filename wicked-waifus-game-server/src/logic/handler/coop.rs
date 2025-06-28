use crate::logic::thread_mgr::NetContext;
use wicked_waifus_protocol::{ErrorCode, LobbyListRequest, LobbyListResponse};

pub fn on_lobby_list_request(
    _ctx: &mut NetContext,
    request: LobbyListRequest,
    response: &mut LobbyListResponse,
) {
    match request.is_friend {
        true => {
            tracing::debug!("Requesting list of friends lobbies");
        }
        false => {
            tracing::debug!("Requesting list of open lobbies");
        }
    }
    response.error_code = ErrorCode::Success.into();
}
