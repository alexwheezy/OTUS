#![allow(unused_variables)]
#![allow(dead_code)]

#[derive(Debug, Clone)]
pub struct Apartament {
    name: String,
    devices: Vec<String>,
}

impl Apartament {
    pub fn new(name: &str, devices: Vec<String>) -> Self {
        assert!(!name.is_empty(), "The apartament must have a name.");
        Self {
            name: name.to_owned(),
            devices,
        }
    }

    ///Return apartament name in house.
    pub fn name(&self) -> &str {
        &self.name
    }

    ///Return all devices in apartament.
    pub fn devices(&self) -> &Vec<String> {
        &self.devices
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn initialize_apartament() -> Apartament {
        let apartament = Apartament::new(
            "Living room",
            vec!["Socket".to_owned(), "Thermo".to_owned()],
        );
        apartament
    }

    #[test]
    fn test_constructed() {
        let apartament = initialize_apartament();
        assert_eq!(apartament.name, "Living room");
        assert_eq!(
            apartament.devices,
            vec!["Socket".to_owned(), "Thermo".to_owned()]
        );
    }

    #[test]
    fn test_correct_name_apartament() {
        let apartament = initialize_apartament();
        assert_eq!(apartament.name(), "Living room");
    }

    #[test]
    fn test_number_of_devices() {
        let apartament = initialize_apartament();
        let expected = vec!["Socket".to_owned(), "Thermo".to_owned()];
        assert_eq!(apartament.devices(), &expected);
    }

    #[test]
    fn test_number_of_rooms() {
        let apartament = initialize_apartament();
        assert!(apartament.devices().len() == 2);

        let apartament = Apartament::new("Living room", vec![]);
        assert!(apartament.devices().is_empty());
    }
}
