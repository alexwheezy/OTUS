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

    ///Returns a list of available rooms in the house.
    pub fn rooms(&self) -> &Rooms {
        &self.rooms
    }

    ///The method adds a new room to the house and returns a mutable reference to the new room.
    pub fn add_room(&mut self, name: &str) -> &mut Room {
        self.rooms
            .entry(name.to_owned())
            .or_insert(Room::new(Devices::new()));
        self.rooms.get_mut(name).unwrap()
    }

    ///Delete the room if it is found.
    ///Removing the same room does not lead to panic.
    pub fn remove_room(&mut self, name: &str) {
        self.rooms.remove(name);
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
        if self.rooms().is_empty() {
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

        for (room, devices) in self.rooms() {
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
        assert!(house.rooms().is_empty());
    }

    #[test]
    fn test_empty_rooms() {
        let house = initialize_house();

        assert!(house.devices("").is_none());
    }

    #[test]
    fn test_number_of_rooms_in_house() {
        let house = initialize_house();

        assert_eq!(house.rooms().len(), 1);
    }

    #[test]
    fn test_number_of_devices_in_rooms() {
        let house = initialize_house();
        let devices = house.devices("Living room");

        assert_eq!(devices.unwrap().len(), 2);
    }

    #[test]
    fn test_another_add_room() {
        let mut house = initialize_house();
        house.add_room("Guest room");

        assert_eq!(house.rooms().len(), 2);
    }

    #[test]
    fn test_same_add_room() {
        let mut house = initialize_house();
        house.add_room("Living room");

        assert_eq!(house.rooms().len(), 1);
    }

    #[test]
    fn test_remove_room() {
        let mut house = initialize_house();
        house.add_room("Living room");
        house.add_room("Guest room");
        house.add_room("Kitchen");

        assert_eq!(house.rooms().len(), 3);

        house.remove_room("Living room");
        assert_eq!(house.rooms().len(), 2);

        house.remove_room("Test room");
        assert_eq!(house.rooms().len(), 2);

        house.remove_room("Guest room");
        house.remove_room("Kitchen");
        house.remove_room("Kitchen");
        house.remove_room("Kitchen");

        assert!(house.rooms().is_empty());
    }
}
