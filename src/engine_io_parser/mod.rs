//! Contains the `EngineIoParser` trait

use crate::engine_io_parser::errors::ParserError;
use crate::engine_io_parser::packet::Packet;

/// The trait of EngineIoParser
/// Protocol Specification: https://github.com/socketio/engine.io-protocol
pub trait EngineIoParser {
    /// Encodes a packet. <packet type id><data>
    /// Binary is encoded in an identical principle
    /// ```rust
    /// # use socket_io_rust::engine_io_parser::engine_io_parser_impl::EngineIOParserImpl;
    /// # use socket_io_rust::engine_io_parser::packet::{Packet, PacketType};
    /// # use socket_io_rust::engine_io_parser::EngineIoParser;
    ///
    /// let data: Vec<u8> = vec![1, 2, 3];
    /// let packet = Packet::new(PacketType::Message, data);
    /// let res = EngineIOParserImpl::encode_packet(packet);
    /// let expected: Vec<u8> = vec![4, 1, 2, 3];
    /// assert_eq!(expected, res)
    /// ```
    fn encode_packet(packet: Packet) -> Vec<u8>;

    /// Decodes a packet
    /// ```rust
    /// # use socket_io_rust::engine_io_parser::engine_io_parser_impl::EngineIOParserImpl;
    /// # use socket_io_rust::engine_io_parser::packet::{Packet, PacketType};
    /// # use socket_io_rust::engine_io_parser::EngineIoParser;
    ///
    /// let encoded_data: Vec<u8> = vec![4, 1, 2, 3];
    /// let packet = EngineIOParserImpl::decode_packet(&encoded_data).unwrap();
    /// let data: Vec<u8> = vec![1, 2, 3];
    /// let expected = Packet::new(PacketType::Message, data);
    /// assert_eq!(expected, packet)
    /// ```
    fn decode_packet(data: &[u8]) -> Result<Packet, ParserError>;
}

pub mod engine_io_parser_impl;
pub mod errors;
pub mod packet;
