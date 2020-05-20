use serde_json::Value;
use crate::socket_io_packet_codec::packet_type::SocketIoPacketType;

#[derive(Debug, PartialEq, Clone)]
pub struct SocketIoPacket {
    pub packet_type: SocketIoPacketType,
    pub nsp: String,
    pub id: Option<u32>,
    pub data: Option<Value>,
}
