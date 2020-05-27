use crate::socket_io_packet_codec::number_util::convert_char_to_number;
use std::convert::TryFrom;

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum SocketIoPacketType {
    Connect = 0,
    Disconnect = 1,
    Event = 2,
    Ack = 3,
    Error = 4,
    BinaryEvent = 5,
    BinaryAck = 6,
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

impl SocketIoPacketType {
    pub fn convert_from_char(c: char) -> Result<Self, ()> {
        match convert_char_to_number(c) {
            Some(n) => SocketIoPacketType::try_from(n),
            None => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::socket_io_packet_codec::packet_type::SocketIoPacketType;
    use crate::socket_io_packet_codec::packet_type::SocketIoPacketType::*;
    use std::convert::TryFrom;

    #[test]
    fn convert_from_char_test() {
        let chars = vec!['0', '1', '2', '3', '4', '5', '6'];
        let types = vec![
            Connect,
            Disconnect,
            Event,
            Ack,
            Error,
            BinaryEvent,
            BinaryAck,
        ];

        for (&left, right) in chars.iter().zip(types) {
            assert_eq!(right, SocketIoPacketType::convert_from_char(left).unwrap());
        }
    }

    #[test]
    fn convert_from_char_test_error() {
        assert!(SocketIoPacketType::convert_from_char('7').is_err());
        assert!(SocketIoPacketType::convert_from_char('a').is_err());
    }

    #[test]
    fn convert_packet_type_to_u8_test() {
        let types = vec![
            Connect,
            Disconnect,
            Event,
            Ack,
            Error,
            BinaryEvent,
            BinaryAck,
        ];
        for (ii, &item) in types.iter().enumerate() {
            assert_eq!(ii as u8, item as u8)
        }
    }

    #[test]
    fn convert_from_u8_to_packet_type_test() {
        let types = vec![
            Connect,
            Disconnect,
            Event,
            Ack,
            Error,
            BinaryEvent,
            BinaryAck,
        ];
        for (ii, &item) in types.iter().enumerate() {
            assert_eq!(item, SocketIoPacketType::try_from(ii as u8).unwrap());
        }
    }

    #[test]
    fn convert_from_u8_to_packet_type_test_null() {
        assert!(SocketIoPacketType::try_from(9).is_err());
        assert!(SocketIoPacketType::try_from(111).is_err());
    }
}
