use crate::engine_io_parser::packet::Packet;
use std::collections::HashMap;
use std::fmt::Error;

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
    fn open(&mut self);
    fn close(&mut self);
    fn send(&mut self, packets: Vec<Packet>);
    fn on_open(&mut self);
    fn on_data(&mut self, data: [u8]);
    fn on_error(&mut self, err: Error);
}
