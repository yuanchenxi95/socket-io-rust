use crate::socket_io_parser::socket_io_packet::SocketIoPacket;

mod number_util;
pub mod socket_io_packet;
pub mod socket_io_packet_type;
pub mod socket_io_parser_impl;

pub trait SocketIoParser {
    fn encode(packet: SocketIoPacket) -> String;
    fn decode(data: String) -> Result<SocketIoPacket, ()>;
}
