use crate::socket_io_packet_codec::socket_io_packet::SocketIoPacket;

pub mod default_packet_codec;
mod number_util;
pub mod socket_io_packet;
pub mod packet_type;

use std::error;
use std::fmt;

pub trait SocketIoPacketCodec {
    type Error: error::Error;

    fn encode(&self, packet: &SocketIoPacket, f: &mut impl fmt::Write) -> Result<(), Self::Error>;
    fn decode(&self, data: &str) -> Result<SocketIoPacket, Self::Error>;

    fn encode_packet_to_string(&self, packet: &SocketIoPacket) -> Result<String, Self::Error> {
        let mut s = String::new();
        self.encode(packet, &mut s).map(|_| s)
    }
}
