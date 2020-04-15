use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum ParserError {
    #[error("Invalid Packet Type: ${0}")]
    InvalidPacketType(u8),
    #[error("Empty Packet")]
    EmptyPacket,
}
