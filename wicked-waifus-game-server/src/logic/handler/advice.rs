use crate::logic::thread_mgr::NetContext;
use wicked_waifus_protocol::{AdviceSetRequest, AdviceSetResponse, ErrorCode};

pub fn on_advice_set_request(
    ctx: &mut NetContext,
    request: AdviceSetRequest,
    response: &mut AdviceSetResponse,
) {
    ctx.player.advise.is_show = request.is_show;

    response.is_show = request.is_show;
    response.error_code = ErrorCode::Success.into();
}
