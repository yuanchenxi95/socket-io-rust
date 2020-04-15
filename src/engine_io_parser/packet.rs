use crate::engine_io_parser::errors::ParserError;
use std::convert::TryFrom;

/// Represents a Packet Type
///
/// 0 open
/// Sent from the server when a new transport is opened (recheck)
///
/// 1 close
/// Request the close of this transport but does not shutdown the connection itself.
///
/// 2 ping
/// Sent by the client. Server should answer with a pong packet containing the same data
///
/// example
///
/// client sends: 2probe
/// server sends: 3probe
/// 3 pong
/// Sent by the server to respond to ping packets.
///
/// 4 message
/// actual message, client and server should call their callbacks with the data.
///
/// example 1
/// server sends: 4HelloWorld
/// client receives and calls callback socket.on('message', function (data) { console.log(data); });
/// example 2
/// client sends: 4HelloWorld
/// server receives and calls callback socket.on('message', function (data) { console.log(data); });
/// 5 upgrade
/// Before engine.io switches a transport, it tests, if server and client can communicate over this transport. If this test succeed, the client sends an upgrade packets which requests the server to flush its cache on the old transport and switch to the new transport.
///
/// 6 noop
/// A noop packet. Used primarily to force a poll cycle when an incoming websocket connection is received.
#[repr(u8)]
#[derive(Debug, PartialEq, Eq)]
pub enum PacketType {
    Open = 0,
    Close = 1,
    Ping = 2,
    Pong = 3,
    Message = 4,
    Upgrade = 5,
    Noop = 6,
}

impl Into<u8> for PacketType {
    fn into(self) -> u8 {
        self as u8
    }
}

impl TryFrom<u8> for PacketType {
    type Error = ParserError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PacketType::Open),
            1 => Ok(PacketType::Close),
            2 => Ok(PacketType::Ping),
            3 => Ok(PacketType::Pong),
            4 => Ok(PacketType::Message),
            5 => Ok(PacketType::Upgrade),
            6 => Ok(PacketType::Noop),
            _ => Err(ParserError::InvalidPacketType(value)),
        }
    }
}

/// Represents a Packet
/// <packet type id><data>
#[derive(Debug, PartialEq, Eq)]
pub struct Packet {
    pub packet_type: PacketType,
    pub data: Vec<u8>,
}

impl Packet {
    pub fn new(packet_type: PacketType, data: Vec<u8>) -> Self {
        Packet { packet_type, data }
    }
}
