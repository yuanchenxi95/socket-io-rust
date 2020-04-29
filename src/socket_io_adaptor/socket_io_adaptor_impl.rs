use std::collections::{HashMap, HashSet};

struct Room {
    sockets: HashSet<String>,
}

impl Room {
    pub fn new() -> Self {
        Room {
            sockets: HashSet::new(),
        }
    }

    pub fn add(&mut self, sid: &str) {
        self.sockets.insert(sid.to_string());
    }

    pub fn delete(&mut self, sid: &str) -> bool {
        self.sockets.remove(sid)
    }

    pub fn length(&self) -> usize {
        self.sockets.len()
    }
}

pub struct SocketIoAdaptor {
    rooms: HashMap<String, Room>,
    sids: HashMap<String, HashSet<String>>,
}

impl Default for SocketIoAdaptor {
    fn default() -> Self {
        SocketIoAdaptor::new()
    }
}

impl SocketIoAdaptor {
    pub fn new() -> Self {
        SocketIoAdaptor {
            rooms: HashMap::new(),
            sids: HashMap::new(),
        }
    }
    fn remove_id_from_room(rooms: &mut HashMap<String, Room>, id: &str, room_id: &str) {
        if let Some(room) = rooms.get_mut(room_id) {
            room.delete(id);
            if room.length() == 0 {
                rooms.remove(room_id);
            }
        }
    }

    pub fn add(&mut self, id: &str, room_id: &str) {
        self.add_all(id, vec![room_id]);
    }

    pub fn add_all(&mut self, id: &str, room_ids: Vec<&str>) {
        for room_id in room_ids {
            let sid = self.sids.entry(id.to_string()).or_insert_with(HashSet::new);
            sid.insert(room_id.to_string());

            let room = self.rooms.entry(room_id.to_string()).or_insert_with(Room::new);
            room.add(id);
        }
    }

    pub fn delete(&mut self, id: &str, room_id: &str) {
        if let Some(room_map) = self.sids.get_mut(id) {
            room_map.remove(room_id);
        }

        SocketIoAdaptor::remove_id_from_room(&mut self.rooms, id, room_id);
    }

    pub fn delete_all(&mut self, id: &str) {
        if let Some(rooms) = self.sids.get(id) {
            for room_id in rooms {
                SocketIoAdaptor::remove_id_from_room(&mut self.rooms, id, room_id);
            }
        }

        self.sids.remove(id);
    }

    /// Get a set of socket id from the given rooms
    pub fn get_all_sids_from_rooms(&self, room_ids: Vec<&str>) -> HashSet<String> {
        let mut sid_set = HashSet::new();
        for room_id in room_ids {
            if let Some(room) = self.rooms.get(room_id) {
                for sid in room.sockets.iter() {
                    // todo check socket connected
                    sid_set.insert(sid.clone());
                }
            }
        }
        sid_set
    }

    /// Get a set of all socket in this adapter
    pub fn get_all_sids(&self) -> HashSet<String> {
        // todo check socket connected
        self.sids.keys().cloned().collect()
    }

    /// Get the set of rooms that a given socket has joined
    pub fn get_socket_rooms(&self, id: &str) -> Option<HashSet<String>> {
        if let Some(room_map) = self.sids.get(id) {
            return Some(room_map.clone());
        }
        None
    }
}

#[cfg(test)]
mod socket_io_adaptor_tests {
    use crate::socket_io_adaptor::socket_io_adaptor_impl::SocketIoAdaptor;

    fn set_up() -> SocketIoAdaptor {

        let mut adaptor = SocketIoAdaptor::new();
        adaptor.add("sid1", "room1");
        adaptor.add("sid1", "room2");
        adaptor.add("sid2", "room1");
        adaptor.add("sid2", "room3");
        adaptor.add("sid3", "room3");
        adaptor
    }

    #[test]
    pub fn add_test() {
        let mut adaptor = SocketIoAdaptor::new();
        adaptor.add("sid1", "room1");

        let rooms = adaptor.sids.get("sid1").unwrap();
        assert!(rooms.contains("room1"));
        assert_eq!(rooms.len(), 1);

        let room = adaptor.rooms.get("room1").unwrap();
        assert!(room.sockets.contains("sid1"));
        assert_eq!(room.sockets.len(), 1);
    }

    #[test]
    pub fn delete_test() {
        let mut adaptor = SocketIoAdaptor::new();
        adaptor.add("sid1", "room1");
        adaptor.add("sid1", "room2");
        adaptor.delete("sid1", "room2");

        let rooms = adaptor.sids.get("sid1").unwrap();
        assert!(rooms.contains("room1"));
        assert!(!rooms.contains("room2"));
        assert_eq!(rooms.len(), 1);

        let room = adaptor.rooms.get("room1").unwrap();
        assert!(room.sockets.contains("sid1"));
        assert_eq!(room.sockets.len(), 1);
    }

    #[test]
    pub fn delete_all_test() {
        let mut adaptor = SocketIoAdaptor::new();
        adaptor.add("sid1", "room1");
        adaptor.add("sid1", "room2");
        adaptor.add("sid2", "room1");
        adaptor.delete_all("sid1");

        assert!(!adaptor.sids.contains_key("sid1"));
        assert!(adaptor.sids.contains_key("sid2"));

        let room1 = adaptor.rooms.get("room1").unwrap();
        assert!(room1.sockets.contains("sid2"));
        assert!(!room1.sockets.contains("sid1"));
    }

    #[test]
    pub fn test_get_all_sids_from_rooms() {
        let adaptor = set_up();

        let sids = adaptor.get_all_sids_from_rooms(vec!["room1", "room2"]);

        assert!(sids.contains("sid1"));
        assert!(sids.contains("sid2"));
        assert_eq!(sids.len(), 2);
    }

    #[test]
    pub fn test_get_all_sids() {
        let adaptor = set_up();
        let sids = adaptor.get_all_sids();

        assert!(sids.contains("sid1"));
        assert!(sids.contains("sid2"));
        assert!(sids.contains("sid3"));
        assert_eq!(sids.len(), 3);
    }
}
