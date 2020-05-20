use crate::socket_io_parser::number_util::convert_char_to_number;
use crate::socket_io_parser::socket_io_packet::SocketIoPacket;
use crate::socket_io_parser::socket_io_packet_type::SocketIoPacketType;
use crate::socket_io_parser::SocketIoParser;

pub struct SocketIoParserImpl;

impl SocketIoParser for SocketIoParserImpl {
    fn encode(packet: SocketIoPacket) -> String {
        let mut builder = String::from("");
        let packet_type_string = packet.packet_type.convert_to_encoded_string();
        builder.push_str(packet_type_string.as_str());
        // todo: handle BINARY_EVENT

        if packet.nsp != "/" {
            builder.push_str(packet.nsp.as_str());
            builder.push_str(",");
        }

        if let Some(id) = packet.id {
            builder.push_str(id.to_string().as_str());
        }

        if let Some(data) = packet.data {
            builder.push_str(data.to_string().as_str());
        }

        builder
    }

    fn decode(data: String) -> Result<SocketIoPacket, ()> {
        if data.len() == 0 {
            return Err(());
        }

        let mut idx = 0;
        let chars: Vec<char> = data.chars().collect();

        // get packet type
        let packet_type = SocketIoPacketType::convert_from_char(chars[idx])?;

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
            return Err(());
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
                    Err(_) => return Err(()),
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
                Err(_) => return Err(()),
            }
        }

        Ok(packet)
    }
}

#[cfg(test)]
mod tests {
    use crate::socket_io_parser::socket_io_packet::SocketIoPacket;
    use crate::socket_io_parser::socket_io_packet_type::SocketIoPacketType;
    use crate::socket_io_parser::socket_io_parser_impl::SocketIoParserImpl;
    use crate::socket_io_parser::SocketIoParser;
    use std::borrow::Borrow;

    #[test]
    fn encode_test_1() {
        let packet_type = SocketIoPacketType::Connect;
        let packet_type_string = packet_type.convert_to_encoded_string();
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

        let encoded_string = SocketIoParserImpl::encode(packet);
        let expected = format!("{}{},{}{}", packet_type_string, nsp, id, data_string);
        assert_eq!(encoded_string, expected)
    }

    #[test]
    fn encode_test_2() {
        let packet_type = SocketIoPacketType::Ack;
        let packet_type_string = packet_type.convert_to_encoded_string();
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

        let encoded_string = SocketIoParserImpl::encode(packet);
        let expected = format!("{}{}", packet_type_string, data_string);
        assert_eq!(encoded_string, expected)
    }

    #[test]
    fn encode_test_3() {
        let packet_type = SocketIoPacketType::Event;
        let packet_type_string = packet_type.convert_to_encoded_string();
        let nsp = "/";

        let packet = SocketIoPacket {
            packet_type,
            nsp: nsp.to_string(),
            id: None,
            data: None,
        };

        let encoded_string = SocketIoParserImpl::encode(packet);
        let expected = format!("{}", packet_type_string);
        assert_eq!(encoded_string, expected)
    }

    #[test]
    fn decode_test_1() {
        let packet_type = SocketIoPacketType::Connect;
        let packet_type_string = packet_type.convert_to_encoded_string();
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
        let decoded = SocketIoParserImpl::decode(input).unwrap();
        assert_eq!(packet, decoded);
    }

    #[test]
    fn decode_test_2() {
        let packet_type = SocketIoPacketType::Connect;
        let packet_type_string = packet_type.convert_to_encoded_string();
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
        let decoded = SocketIoParserImpl::decode(input).unwrap();
        assert_eq!(packet, decoded);
    }


    #[test]
    fn decode_test_3() {
        let packet_type = SocketIoPacketType::Connect;
        let packet_type_string = packet_type.convert_to_encoded_string();
        let nsp = "/hello";

        let packet = SocketIoPacket {
            packet_type,
            nsp: nsp.to_string(),
            id: None,
            data: None,
        };

        let input = format!("{}{},", packet_type_string, nsp);
        let decoded = SocketIoParserImpl::decode(input).unwrap();
        assert_eq!(packet, decoded);
    }

    #[test]
    fn decode_test_4() {
        let packet_type = SocketIoPacketType::Connect;
        let packet_type_string = packet_type.convert_to_encoded_string();
        let nsp = "/hello";

        let packet = SocketIoPacket {
            packet_type,
            nsp: nsp.to_string(),
            id: None,
            data: None,
        };

        let input = format!("{}{}", packet_type_string, nsp);
        let decoded = SocketIoParserImpl::decode(input).unwrap();
        assert_eq!(packet, decoded);
    }

    #[test]
    fn decode_test_5() {
        let packet_type = SocketIoPacketType::Connect;
        let packet_type_string = packet_type.convert_to_encoded_string();
        let nsp = "/";

        let packet = SocketIoPacket {
            packet_type,
            nsp: nsp.to_string(),
            id: None,
            data: None,
        };

        let input = format!("{}", packet_type_string);
        let decoded = SocketIoParserImpl::decode(input).unwrap();
        assert_eq!(packet, decoded);
    }

    #[test]
    fn decode_test_6() {
        let packet_type = SocketIoPacketType::Connect;
        let packet_type_string = packet_type.convert_to_encoded_string();
        let nsp = "/";
        let id = 100;
        let packet = SocketIoPacket {
            packet_type,
            nsp: nsp.to_string(),
            id: Some(id),
            data: None,
        };

        let input = format!("{}{}", packet_type_string, id);
        let decoded = SocketIoParserImpl::decode(input).unwrap();
        assert_eq!(packet, decoded);
    }

    #[test]
    fn decode_test_7() {
        let packet_type = SocketIoPacketType::Connect;
        let packet_type_string = packet_type.convert_to_encoded_string();
        let nsp = "/";
        let id = 0;
        let packet = SocketIoPacket {
            packet_type,
            nsp: nsp.to_string(),
            id: Some(id),
            data: None,
        };

        let input = format!("{}{}", packet_type_string, id);
        let decoded = SocketIoParserImpl::decode(input).unwrap();
        assert_eq!(packet, decoded);
    }
}
