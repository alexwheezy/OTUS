#![allow(unused_variables)]
#![allow(dead_code)]

pub mod apartament;

use crate::house::apartament::Apartament;
use crate::providers::info::DeviceInfoProvider;

#[derive(Debug, Clone)]
pub struct House {
    name: String,
    apartaments: Vec<Apartament>,
}

impl House {
    pub fn new(name: &str, apartaments: Vec<Apartament>) -> Self {
        assert!(!name.is_empty(), "House must be the name.");
        Self {
            name: name.to_owned(),
            apartaments,
        }
    }

    ///Return number of rooms in the house.
    pub fn get_rooms(&self) -> Vec<String> {
        self.apartaments
            .iter()
            .map(|current_apartament| current_apartament.name().to_owned())
            .collect()
    }

    ///Return number of devices in the apartament.
    pub fn devices(&self, apartament: &str) -> Vec<String> {
        if apartament.is_empty() {
            return vec![];
        }

        self.apartaments
            .iter()
            .filter(|&current_apartament| current_apartament.name() == apartament)
            .flat_map(|apartament| apartament.devices().clone())
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
            apartaments: vec![initialize_apartament],
        };

        house
    }

    fn test_number_of_rooms() {
        let house = initialize_house();
        let expected = vec!["Living room"];
        let output = house.get_rooms();
        assert_eq!(output, expected);
        assert_ne!(output.len(), 0);
    }

    #[test]
    fn test_number_of_devices() {
        let house = initialize_house();
        let expected = vec!["Socket".to_owned(), "Thermo".to_owned()];
        let output = house.devices("Living room");
        assert_eq!(output, expected);
        assert_ne!(output.len(), 0);
    }
}
