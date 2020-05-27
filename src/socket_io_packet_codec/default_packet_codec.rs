use crate::socket_io_packet_codec::number_util::convert_char_to_number;
use crate::socket_io_packet_codec::packet_type::SocketIoPacketType;
use crate::socket_io_packet_codec::socket_io_packet::SocketIoPacket;
use crate::socket_io_packet_codec::{SocketIoPacketCodec, PacketCodecError};
use core::fmt;


#[derive(Default, Debug)]
pub struct DefaultPacketCodec;


impl SocketIoPacketCodec for DefaultPacketCodec {
    type Error = PacketCodecError;

    fn encode(&self, packet: &SocketIoPacket, f: &mut impl fmt::Write) -> Result<(), Self::Error> {
        write!(f, "{}", packet.packet_type as u8)?;
        // todo: handle BINARY_EVENT

        if packet.nsp != "/" {
            write!(f, "{}", packet.nsp)?;
            write!(f, ",")?;
        }

        if let Some(id) = packet.id {
            write!(f, "{}", id)?;
        }

        if let Some(data) = &packet.data {
            write!(f, "{}", data)?;
        }
        Ok(())
    }

    fn decode(&self, data: &str) -> Result<SocketIoPacket, Self::Error> {
        if data.is_empty() {
            return Err(PacketCodecError::DecodeError("Data is empty".to_string()));
        }

        let mut idx = 0;
        let chars: Vec<char> = data.chars().collect();

        // get packet type
        let packet_type = SocketIoPacketType::convert_from_char(chars[idx])
            .map_err(|_| PacketCodecError::DecodeError("Unknown packet type".to_string()))?;

        let mut packet = SocketIoPacket {
            packet_type,
            nsp: String::new(),
            id: None,
            data: None,
        };

        // todo handle BINARY_EVENT
        if packet.packet_type == SocketIoPacketType::BinaryAck
            || packet.packet_type == SocketIoPacketType::BinaryEvent
        {
            return Err(PacketCodecError::DecodeError("Binary not supported".to_string()));
        }

        // get packet namespace
        if chars.len() > idx + 1 && *chars.get(idx + 1).unwrap() == '/' {
            loop {
                idx += 1;
                if idx >= chars.len() {
                    break;
                }
                let &current_char = chars.get(idx).unwrap();
                if current_char == ',' {
                    break;
                }
                packet.nsp.push(current_char);
            }
        } else {
            packet.nsp.push('/');
        }

        // lookup id
        let next_idx = idx + 1;
        if chars.len() > next_idx {
            let mut id_builder = String::new();
            // check the first char is a number between 0 - 10
            let &next_char = chars.get(next_idx).unwrap();
            if convert_char_to_number(next_char) != None {
                id_builder.push(next_char);
                idx += 1;
                loop {
                    idx += 1;

                    // idx exceeds the length
                    if idx >= chars.len() {
                        break;
                    }
                    // check next char is number
                    let &c = chars.get(idx).unwrap();

                    // if not, minus 1 to idx
                    if convert_char_to_number(c) == None {
                        idx -= 1;
                        break;
                    }
                    id_builder.push(c);
                }

                match id_builder.parse::<u32>() {
                    Ok(n) => packet.id = Some(n),
                    Err(_) => return Err(PacketCodecError::DecodeError("Unable to parse ID".to_string())),
                };
            }
        }

        // parse json data
        idx += 1;
        if chars.len() > idx {
            match serde_json::from_str(&data[idx..]) {
                Ok(v) => {
                    packet.data = Some(v);
                }
                Err(e) => return Err(PacketCodecError::DecodeError(e.to_string())),
            }
        }

        Ok(packet)
    }
}

#[cfg(test)]
mod tests {
    use crate::socket_io_packet_codec::default_packet_codec::DefaultPacketCodec;
    use crate::socket_io_packet_codec::packet_type::SocketIoPacketType;
    use crate::socket_io_packet_codec::socket_io_packet::SocketIoPacket;
    use crate::socket_io_packet_codec::SocketIoPacketCodec;

    fn convert_type_to_string(packet: SocketIoPacketType) -> String {
        format!("{}", packet as u8)
    }

    #[test]
    fn encode_test_1() {
        let packet_type = SocketIoPacketType::Connect;
        let packet_type_string = convert_type_to_string(packet_type);
        let nsp = "/hello";
        let id = 100;
        let data = serde_json::json!({
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        });
        let data_string = data.to_string();

        let packet = SocketIoPacket {
            packet_type,
            nsp: nsp.to_string(),
            id: Some(id),
            data: Some(data),
        };

        let encoded_string = DefaultPacketCodec.encode_packet_to_string(&packet).unwrap();
        let expected = format!("{}{},{}{}", packet_type_string, nsp, id, data_string);
        assert_eq!(encoded_string, expected)
    }

    #[test]
    fn encode_test_2() {
        let packet_type = SocketIoPacketType::Ack;
        let packet_type_string = convert_type_to_string(packet_type);
        let nsp = "/";
        let data = serde_json::json!({
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        });
        let data_string = data.to_string();

        let packet = SocketIoPacket {
            packet_type,
            nsp: nsp.to_string(),
            id: None,
            data: Some(data),
        };

        let encoded_string = DefaultPacketCodec.encode_packet_to_string(&packet).unwrap();
        let expected = format!("{}{}", packet_type_string, data_string);
        assert_eq!(encoded_string, expected)
    }

    #[test]
    fn encode_test_3() {
        let packet_type = SocketIoPacketType::Event;
        let packet_type_string = convert_type_to_string(packet_type);
        let nsp = "/";

        let packet = SocketIoPacket {
            packet_type,
            nsp: nsp.to_string(),
            id: None,
            data: None,
        };

        let encoded_string = DefaultPacketCodec.encode_packet_to_string(&packet).unwrap();
        let expected = packet_type_string;
        assert_eq!(encoded_string, expected)
    }

    #[test]
    fn decode_test_1() {
        let packet_type = SocketIoPacketType::Connect;
        let packet_type_string = convert_type_to_string(packet_type);
        let nsp = "/hello";
        let id = 100;
        let data = serde_json::json!({
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        });
        let data_string = data.to_string();

        let packet = SocketIoPacket {
            packet_type,
            nsp: nsp.to_string(),
            id: Some(id),
            data: Some(data),
        };

        let input = format!("{}{},{}{}", packet_type_string, nsp, id, data_string);
        let decoded = DefaultPacketCodec.decode(&input).unwrap();
        assert_eq!(packet, decoded);
    }

    #[test]
    fn decode_test_2() {
        let packet_type = SocketIoPacketType::Connect;
        let packet_type_string = convert_type_to_string(packet_type);
        let nsp = "/hello";
        let data = serde_json::json!({
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        });
        let data_string = data.to_string();

        let packet = SocketIoPacket {
            packet_type,
            nsp: nsp.to_string(),
            id: None,
            data: Some(data),
        };

        let input = format!("{}{},{}", packet_type_string, nsp, data_string);
        let decoded = DefaultPacketCodec.decode(&input).unwrap();
        assert_eq!(packet, decoded);
    }

    #[test]
    fn decode_test_3() {
        let packet_type = SocketIoPacketType::Connect;
        let packet_type_string = convert_type_to_string(packet_type);
        let nsp = "/hello";

        let packet = SocketIoPacket {
            packet_type,
            nsp: nsp.to_string(),
            id: None,
            data: None,
        };

        let input = format!("{}{},", packet_type_string, nsp);
        let decoded = DefaultPacketCodec.decode(&input).unwrap();
        assert_eq!(packet, decoded);
    }

    #[test]
    fn decode_test_4() {
        let packet_type = SocketIoPacketType::Connect;
        let packet_type_string = convert_type_to_string(packet_type);
        let nsp = "/hello";

        let packet = SocketIoPacket {
            packet_type,
            nsp: nsp.to_string(),
            id: None,
            data: None,
        };

        let input = format!("{}{}", packet_type_string, nsp);
        let decoded = DefaultPacketCodec.decode(&input).unwrap();
        assert_eq!(packet, decoded);
    }

    #[test]
    fn decode_test_5() {
        let packet_type = SocketIoPacketType::Connect;
        let packet_type_string = convert_type_to_string(packet_type);
        let nsp = "/";

        let packet = SocketIoPacket {
            packet_type,
            nsp: nsp.to_string(),
            id: None,
            data: None,
        };

        let input = packet_type_string;
        let decoded = DefaultPacketCodec.decode(&input).unwrap();
        assert_eq!(packet, decoded);
    }

    #[test]
    fn decode_test_6() {
        let packet_type = SocketIoPacketType::Connect;
        let packet_type_string = convert_type_to_string(packet_type);
        let nsp = "/";
        let id = 100;
        let packet = SocketIoPacket {
            packet_type,
            nsp: nsp.to_string(),
            id: Some(id),
            data: None,
        };

        let input = format!("{}{}", packet_type_string, id);
        let decoded = DefaultPacketCodec.decode(&input).unwrap();
        assert_eq!(packet, decoded);
    }

    #[test]
    fn decode_test_7() {
        let packet_type = SocketIoPacketType::Connect;
        let packet_type_string = convert_type_to_string(packet_type);
        let nsp = "/";
        let id = 0;
        let packet = SocketIoPacket {
            packet_type,
            nsp: nsp.to_string(),
            id: Some(id),
            data: None,
        };

        let input = format!("{}{}", packet_type_string, id);
        let decoded = DefaultPacketCodec.decode(&input).unwrap();
        assert_eq!(packet, decoded);
    }
}
