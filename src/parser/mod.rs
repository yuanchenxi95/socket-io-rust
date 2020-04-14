use crate::data::errors::ParserError;
use crate::data::Packet;

pub trait SocketIoParser {
    fn encode_packet(packet: Packet) -> Vec<u8>;
    fn decode_packet(data: &[u8]) -> Result<Packet, ParserError>;
}

pub mod parser_impl;
