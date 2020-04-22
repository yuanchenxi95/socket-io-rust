use crate::engine_io_parser::packet::Packet;
use std::collections::HashMap;

pub enum TransportReadyState {
    OPENING,
    OPEN,
    CLOSED,
    PAUSED,
}

pub struct TransportOptions {
    pub hostname: String,
    pub path: String,
    pub port: usize,
    pub policy_port: usize,
    pub timestamp_param: String,
    pub secure: bool,
    pub timestamp_requests: bool,
    pub query: HashMap<String, String>,
}

pub trait Transport {
    fn open() -> Self;
    fn close() -> Self;
    fn send(packets: Vec<Packet>);
}
