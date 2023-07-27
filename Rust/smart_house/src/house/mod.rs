#![allow(unused_variables)]
#![allow(dead_code)]

pub mod room;

use std::error::Error;

use crate::house::room::Room;
use crate::providers::info::DeviceInfoProvider;

#[derive(Debug, Clone)]
pub struct House {
    name: String,
    rooms: Vec<Room>,
}

impl House {
    pub fn new(name: &str, rooms: Vec<Room>) -> Self {
        assert!(!name.is_empty(), "House must be the name.");
        Self {
            name: name.to_owned(),
            rooms,
        }
    }

    ///Return number of rooms in the house.
    pub fn get_rooms(&self) -> Vec<String> {
        self.rooms
            .iter()
            .map(|current_room| current_room.name().to_owned())
            .collect::<Vec<String>>()
    }

    ///Return number of devices in the room.
    pub fn devices(&self, room: &str) -> Vec<String> {
        self.rooms
            .iter()
            .filter(|&current_room| current_room.name() == room)
            .flat_map(|room| room.devices().clone())
            .collect::<Vec<String>>()
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

        for room in self.get_rooms() {
            report.push_str(&format!(
                r#"

        Room: [{room}]"#,
            ));
            match self.devices(&room).is_empty() {
                false => {
                    self.devices(&room)
                        .iter()
                        .for_each(|device| (device_status(&mut report, device)));
                }
                true => report.push_str(
                    r#"
        Info: Not enough information to report
        Devices were not found in the room"#,
                ),
            }
        }
        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn initialize_house() -> House {
        let initialize_rooms = Room::new(
            "Living room",
            vec!["Socket".to_owned(), "Thermo".to_owned()],
        );
        let house = House {
            name: "Paradise".to_owned(),
            rooms: vec![initialize_rooms],
        };
        house
    }

    #[test]
    fn test_empty_house() {
        let house = House {
            name: "Paradise".to_owned(),
            rooms: vec![],
        };
        assert!(house.get_rooms().is_empty());
    }

    #[test]
    fn test_empty_rooms() {
        let house = initialize_house();
        assert!(house.devices("").is_empty());
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
        assert_eq!(devices.len(), 2);
    }
}
