use actix::Message;
use std::io;

pub(crate) struct SocketMessage {
    pub(crate) event_name: String,
    pub(crate) content: String,
}

impl Message for SocketMessage {
    type Result = Result<(), io::Error>;
}
