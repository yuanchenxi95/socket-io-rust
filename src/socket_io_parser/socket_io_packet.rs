use crate::socket_io_parser::socket_io_packet_type::SocketIoPacketType;
use serde_json::Value;

#[derive(Debug, PartialEq, Clone)]
pub struct SocketIoPacket {
    pub packet_type: SocketIoPacketType,
    pub nsp: String,
    pub id: Option<u32>,
    pub data: Option<Value>,
}
