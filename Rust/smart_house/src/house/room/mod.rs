#![allow(unused_variables)]
#![allow(dead_code)]

#[derive(Debug, Clone)]
pub struct Room {
    name: String,
    devices: Vec<String>,
}

impl Room {
    pub fn new(name: &str, devices: Vec<String>) -> Self {
        assert!(!name.is_empty(), "The room must have a name.");
        Self {
            name: name.to_owned(),
            devices,
        }
    }

    ///Return room name in the house.
    pub fn name(&self) -> &str {
        &self.name
    }

    ///Return all devices in the house.
    pub fn devices(&self) -> &Vec<String> {
        &self.devices
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn initialize_room() -> Room {
        let room = Room::new(
            "Living room",
            vec!["Socket".to_owned(), "Thermo".to_owned()],
        );
        room
    }

    #[test]
    fn test_correct_name_room() {
        let room = initialize_room();
        assert_eq!(room.name(), "Living room");
    }

    #[test]
    fn test_some_devices_in_room() {
        let room = initialize_room();
        let expected = vec!["Socket".to_owned(), "Thermo".to_owned()];
        assert_eq!(room.devices(), &expected);
    }

    #[test]
    fn test_none_devices_in_room() {
        let room = Room::new("Living room", vec![]);
        assert_eq!(room.devices(), &Vec::<String>::new());
    }

    #[test]
    fn test_number_of_devices() {
        let room = initialize_room();
        let expected = vec!["Socket".to_owned(), "Thermo".to_owned()];
        assert_eq!(room.devices().len(), 2);
    }
}
