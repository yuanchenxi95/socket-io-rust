use crate::socket_io_packet_codec::socket_io_packet::SocketIoPacket;

pub mod default_packet_codec;
mod number_util;
pub mod packet_type;
pub mod socket_io_packet;

use thiserror::Error;
use std::{fmt, error};

#[derive(Debug, Error)]
pub enum PacketCodecError {
    #[error("Decoding Socket IO packet Error: {}", 0)]
    DecodeError(String),
    #[error("Encoding Socket IO packet Error: {}", 0)]
    EncodeError(#[from] fmt::Error),
}

pub trait SocketIoPacketCodec {
    type Error: error::Error;

    fn encode(&self, packet: &SocketIoPacket, f: &mut impl fmt::Write) -> Result<(), Self::Error>;
    fn decode(&self, data: &str) -> Result<SocketIoPacket, Self::Error>;

}


pub(crate) trait SocketIoPacketCodecMono {
    type Error;
    fn encode_mono(&self, packet: &SocketIoPacket, f: &mut String) -> Result<(), Self::Error>;
    fn encode_to_string_mono(&self, packet: &SocketIoPacket) -> Result<String, Self::Error>;
    fn decode_mon(&self, data: &str) -> Result<SocketIoPacket, Self::Error>;
}

impl<T: SocketIoPacketCodec> SocketIoPacketCodecMono for T {
    type Error = T::Error;

    fn encode_mono(&self, packet: &SocketIoPacket, f: &mut String) -> Result<(), Self::Error> {
        self.encode(packet, f)
    }

    fn encode_to_string_mono(&self, packet: &SocketIoPacket) -> Result<String, Self::Error> {
        let mut s = String::new();
        self.encode(packet, &mut s).map(|_| s)
    }

    fn decode_mon(&self, data: &str) -> Result<SocketIoPacket, Self::Error> {
        self.decode(data)
    }
}