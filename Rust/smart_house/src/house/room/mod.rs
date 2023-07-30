#![allow(unused_variables)]
#![allow(dead_code)]

use std::collections::HashSet;
pub type Devices = HashSet<String>;

#[derive(Debug, Clone)]
pub struct Room {
    devices: Devices,
}

impl Room {
    pub fn new(devices: Devices) -> Self {
        Self { devices }
    }

    ///Return all devices in the house.
    pub fn devices(&self) -> &Devices {
        &self.devices
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn initialize_room() -> Room {
        Room::new(Devices::from(["Socket".to_owned(), "Thermo".to_owned()]))
    }

    #[test]
    fn test_some_devices_in_room() {
        let room = initialize_room();
        let expected = Devices::from(["Socket".to_owned(), "Thermo".to_owned()]);
        assert_eq!(room.devices(), &expected);
    }

    #[test]
    fn test_none_devices_in_room() {
        let room = Room::new(Devices::new());
        assert_eq!(room.devices(), &Devices::new());
    }

    #[test]
    fn test_number_of_devices() {
        let room = initialize_room();
        let expected = vec!["Socket".to_owned(), "Thermo".to_owned()];
        assert_eq!(room.devices().len(), 2);
    }
}
