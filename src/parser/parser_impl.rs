use crate::data::errors::ParserError;
use crate::data::{Packet, PacketType};
use crate::parser::SocketIoParser;
use std::convert::TryFrom;

pub struct SocketIOParserImpl {}

impl SocketIoParser for SocketIOParserImpl {
    fn encode_packet(packet: Packet) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::with_capacity(packet.data.len() + 1);

        buffer.push(packet.packet_type.into());
        for item in packet.data {
            buffer.push(item);
        }

        buffer
    }

    fn decode_packet(data: &[u8]) -> Result<Packet, ParserError> {
        if data.is_empty() {
            return Err(ParserError::EmptyPacket);
        }
        let packet_type = PacketType::try_from(data[0])?;
        let packet_data_len = data.len() - 1;
        let mut packet_data = Vec::with_capacity(packet_data_len);
        for ii in 0..packet_data_len {
            packet_data.push(data[ii + 1]);
        }
        Ok(Packet::new(packet_type, packet_data))
    }
}

#[cfg(test)]
mod tests {
    use crate::data::{Packet, PacketType};
    use crate::parser::parser_impl::SocketIOParserImpl;
    use crate::parser::SocketIoParser;

    #[test]
    fn encode_packet_test() {
        let data: Vec<u8> = vec![1, 2, 3];
        let packet = Packet::new(PacketType::Open, data);
        let res = SocketIOParserImpl::encode_packet(packet);
        let expected: Vec<u8> = vec![0, 1, 2, 3];
        assert_eq!(expected, res)
    }

    #[test]
    fn decode_packet_test() {
        let data: Vec<u8> = vec![0, 1, 2, 3];
        let packet = SocketIOParserImpl::decode_packet(&data).unwrap();
        let expected = Packet::new(PacketType::Open, vec![1, 2, 3]);
        assert_eq!(expected, packet);
    }
}
