use crate::engine_io_parser::packet::Packet;
use crate::engine_io_parser::errors::ParserError;

pub trait EngineIoParser {
    fn encode_packet(packet: Packet) -> Vec<u8>;
    fn decode_packet(data: &[u8]) -> Result<Packet, ParserError>;
}

pub mod engine_io_parser_impl;
pub mod errors;
pub mod packet;
