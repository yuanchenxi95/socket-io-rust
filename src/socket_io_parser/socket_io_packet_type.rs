use crate::socket_io_parser::number_util::convert_char_to_number;
use std::convert::TryFrom;

#[derive(Debug, PartialOrd, PartialEq)]
pub enum SocketIoPacketType {
    Connect = 0,
    Disconnect = 1,
    Event = 2,
    Ack = 3,
    Error = 4,
    BinaryEvent = 5,
    BinaryAck = 6,
}

impl SocketIoPacketType {
    pub fn convert_from_char(c: char) -> Result<Self, ()> {
        match convert_char_to_number(c) {
            Some(n) => SocketIoPacketType::try_from(n),
            None => Err(()),
        }
    }
}

impl Into<u8> for SocketIoPacketType {
    fn into(self) -> u8 {
        self as u8
    }
}

impl TryFrom<u8> for SocketIoPacketType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, ()> {
        match value {
            0 => Ok(SocketIoPacketType::Connect),
            1 => Ok(SocketIoPacketType::Disconnect),
            2 => Ok(SocketIoPacketType::Event),
            3 => Ok(SocketIoPacketType::Ack),
            4 => Ok(SocketIoPacketType::Error),
            5 => Ok(SocketIoPacketType::BinaryEvent),
            6 => Ok(SocketIoPacketType::BinaryAck),
            _ => Err(()),
        }
    }
}
