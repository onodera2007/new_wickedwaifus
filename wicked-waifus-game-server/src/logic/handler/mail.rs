use wicked_waifus_protocol::{MailBind, MailBindInfoRequest, MailBindInfoResponse};

use crate::logic::thread_mgr::NetContext;

pub fn on_mail_bind_info_request(
    _: &NetContext,
    _: MailBindInfoRequest,
    response: &mut MailBindInfoResponse,
) {
    // TODO: Implement this
    response.mail_bind = Some(MailBind {
        is_bind: true,
        is_reward: true,
        close_time: -1,
    });
}
