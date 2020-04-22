use crate::engine_io_parser::packet::Packet;
use crate::engine_io_server::transport::{TransportOptions, TransportReadyState};
use std::collections::HashMap;
#[allow(dead_code)]
pub struct Socket {
    ready_state: TransportReadyState,
    socket_options: TransportOptions,
    prior_websocket_success: bool,
    prev_buffer_len: u32,
    ping_interval: u64,
    ping_timeout: u64,
    id: String,
    hostname: String,
    path: String,
    transports: Vec<String>,
    transport_options_map: HashMap<String, TransportOptions>,
    upgrades: Vec<String>,
    write_buffer: Vec<Packet>,
}
