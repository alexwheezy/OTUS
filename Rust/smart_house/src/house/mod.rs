#![allow(unused_variables)]
#![allow(dead_code)]

pub mod apartament;

use crate::house::apartament::Apartament;
use crate::providers::info::DeviceInfoProvider;

#[derive(Debug, Clone)]
pub struct House {
    name: String,
    rooms: Vec<Apartament>,
}

impl House {
    pub fn new(name: &str, rooms: Vec<Apartament>) -> Self {
        Self {
            name: name.to_owned(),
            rooms,
        }
    }

    ///Return list of rooms in the house.
    pub fn get_rooms(&self) -> Vec<String> {
        self.rooms
            .iter()
            .map(|current_room| current_room.name().to_owned())
            .collect()
    }

    ///Return list of devices in the room.
    pub fn devices(&self, room: &str) -> Vec<String> {
        self.rooms
            .iter()
            .filter(|&current_room| current_room.name() == room)
            .flat_map(|room| room.devices().clone())
            .collect()
    }

    ///Text report on the status of devices in the house.
    pub fn create_report(&self, provider: &impl DeviceInfoProvider) -> String {
        //Report header
        let mut report = format!("\n{:>12}: [{}]\n", "House", &self.name);

        for room in self.get_rooms() {
            report.push_str(&format!("\n{:>12}: [{}]", "Apartament", room));
            for device in self.devices(&room) {
                report.push_str(&provider.status(&device));
            }
        }
        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn initialize_house() -> House {
        let initialize_apartament = Apartament::new(
            "Living room",
            vec!["Socket".to_owned(), "Thermo".to_owned()],
        );

        let house = House {
            name: "Paradise".to_owned(),
            rooms: vec![initialize_apartament],
        };

        house
    }

    #[test]
    fn test_constructed() {
        let _house = initialize_house();
    }

    #[test]
    fn test_get_rooms() {
        let house = initialize_house();
        let expected = vec!["Living room"];
        let output = house.get_rooms();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_get_devices() {
        let house = initialize_house();
        let expected = vec!["Socket".to_owned(), "Thermo".to_owned()];
        let output = house.devices("Living room");
        assert_eq!(output, expected);
    }
}