#![allow(unused_variables)]
#![allow(dead_code)]

pub mod room;

use crate::house::room::{Devices, Room};
use crate::providers::info::DeviceInfoProvider;

use std::collections::HashMap;
use std::error::Error;

pub type Rooms = HashMap<String, Room>;

#[derive(Debug, Clone)]
pub struct House {
    name: String,
    rooms: Rooms,
}

impl House {
    pub fn new(name: &str, rooms: Rooms) -> Self {
        assert!(!name.is_empty(), "House must be the name.");
        Self {
            name: name.to_owned(),
            rooms,
        }
    }

    pub fn add_room(&mut self, name: String) {
        todo!("Not implemented add room in the house");
    }

    pub fn remove_room(&mut self, name: String) {
        todo!("Not implemented remove room in the house");
    }

    ///Return number of rooms in the house.
    pub fn get_rooms(&self) -> &Rooms {
        &self.rooms
    }

    pub fn add_device(&mut self, room: Room, device: String) {
        todo!("Not implemented add device in the room");
    }

    pub fn remove_device(&mut self, room: Room, device: String) {
        todo!("Not implemented remove device in the room");
    }

    ///Return number of devices in the room.
    pub fn devices(&self, room: &str) -> Option<&Devices> {
        match self.rooms.get(room) {
            Some(room) => Some(room.devices()),
            _ => None,
        }
    }

    ///Text report on the status of devices in the house.
    pub fn create_report(&self, provider: &impl DeviceInfoProvider) -> String {
        //Report header
        let mut report = format!(
            r#"
       House: [{name}]"#,
            name = &self.name
        );
        if self.get_rooms().is_empty() {
            report.push_str(
                r#"
        Info: Not enough information to report
        Living quarters in the house were not found"#,
            );
            return report;
        }

        let device_status = |report: &mut String, device: &str| match provider.status(device) {
            Ok(provider) => report.push_str(&provider),
            Err(err) => {
                report.push_str(&format!("\n{:>12}: {}\n", "Device", err.source().unwrap()))
            }
        };

        for (room, devices) in self.get_rooms() {
            report.push_str(&format!(
                r#"

        Room: [{room}]
        "#,
            ));
            if let Some(room) = self.devices(room) {
                if !room.is_empty() {
                    room.iter()
                        .for_each(|device| (device_status(&mut report, device)));
                } else {
                    report.push_str(
                        r#"
        Info: Not enough information to report
        Devices were not found in the room"#,
                    );
                }
            }
        }
        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn initialize_house() -> House {
        let initialize_devices =
            Room::new(Devices::from(["Socket".to_owned(), "Thermo".to_owned()]));
        let house = House {
            name: "Paradise".to_owned(),
            rooms: HashMap::from([("Living room".to_owned(), initialize_devices)]),
        };
        house
    }

    #[test]
    fn test_empty_house() {
        let house = House {
            name: "Paradise".to_owned(),
            rooms: HashMap::new(),
        };
        assert!(house.get_rooms().is_empty());
    }

    #[test]
    fn test_empty_rooms() {
        let house = initialize_house();
        assert!(house.devices("").is_none());
    }

    #[test]
    fn test_number_of_rooms_in_house() {
        let house = initialize_house();
        let expected = vec!["Living room"];
        assert_eq!(house.get_rooms().len(), 1);
    }

    #[test]
    fn test_number_of_devices_in_rooms() {
        let house = initialize_house();
        let expected = vec!["Socket".to_owned(), "Thermo".to_owned()];
        let devices = house.devices("Living room");
        assert_eq!(devices.unwrap().len(), 2);
    }
}
