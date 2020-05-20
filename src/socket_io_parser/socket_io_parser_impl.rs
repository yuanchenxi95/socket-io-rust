use crate::socket_io_parser::number_util::convert_char_to_number;
use crate::socket_io_parser::socket_io_packet::SocketIoPacket;
use crate::socket_io_parser::socket_io_packet_type::SocketIoPacketType;
use crate::socket_io_parser::SocketIoParser;

pub struct SocketIoParserImpl;

impl SocketIoParser for SocketIoParserImpl {
    fn encode(packet: SocketIoPacket) -> String {
        let mut builder = String::from("");
        let packet_type_num: u8 = packet.packet_type.into();
        builder.push_str(packet_type_num.to_string().as_str());
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
                    if convert_char_to_number(c) != None {
                        idx -= 1;
                    }
                    id_builder.push(c);
                }
            }

            match id_builder.parse::<u32>() {
                Ok(n) => packet.id = Some(n),
                Err(_) => return Err(()),
            };
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
