#![allow(unused_variables)]
#![allow(dead_code)]

use serde::Deserialize;
use std::collections::HashSet;

pub type Devices = HashSet<String>;

#[derive(Debug, Clone, Deserialize)]
pub struct Room {
    devices: Devices,
}

impl Room {
    /// When creating a new room in the constructor, you can specify a list of required devices.
    pub fn new(devices: Devices) -> Self {
        Self { devices }
    }

    /// Returns a list of available devices in the room.
    pub fn devices(&self) -> &Devices {
        &self.devices
    }

    /// Returns the device name if found.
    pub fn get_device(&self, name: &str) -> Option<&str> {
        self.devices.get(name).map(|s| s.as_str())
    }

    /// Adds a new device.
    /// If the device already exists, then the method does not change the length of the device list.
    pub fn add_device(&mut self, device: &str) -> &mut Self {
        self.devices.insert(device.to_owned());
        self
    }

    /// Delete the device if it is found.
    /// Removing the same device does not lead to panic.
    pub fn remove_device(&mut self, device: &str) {
        self.devices.remove(&device.to_owned());
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

    #[test]
    fn test_another_add_device() {
        let mut room = initialize_room();
        room.add_device("Socket1");
        room.add_device("Thermo1");

        assert_eq!(room.devices().len(), 4);
    }

    #[test]
    fn test_same_add_device() {
        let mut room = initialize_room();
        room.add_device("Socket");
        room.add_device("Thermo");

        assert_eq!(room.devices().len(), 2);
    }

    #[test]
    fn test_get_device() {
        let mut room = initialize_room();
        room.add_device("Socket");
        room.add_device("Thermo");

        assert_eq!(room.get_device("Socket"), Some("Socket"));
        assert_eq!(room.get_device("Socket42"), None);
    }

    #[test]
    fn test_remove_devices() {
        let mut room = initialize_room();
        // Another device
        room.remove_device("Huawei");
        assert_eq!(room.devices.len(), 2);

        room.remove_device("Socket");
        assert_eq!(room.devices.len(), 1);

        room.remove_device("Thermo");
        room.remove_device("Thermo");

        assert!(room.devices.is_empty());
    }
}
