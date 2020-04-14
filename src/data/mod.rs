use crate::data::errors::ParserError;
use std::convert::TryFrom;

pub mod errors;

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
