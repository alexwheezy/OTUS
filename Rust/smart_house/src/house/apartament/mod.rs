#![allow(unused_variables)]
#![allow(dead_code)]

#[derive(Debug, Clone)]
pub struct Apartament {
    name: String,
    devices: Vec<String>,
}

impl Apartament {
    pub fn new(name: &str, devices: Vec<String>) -> Self {
        Self {
            name: name.to_owned(),
            devices,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn devices(&self) -> &Vec<String> {
        &self.devices
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_apartament() -> Apartament {
        let apartament = Apartament::new(
            "Living room",
            vec!["Socket".to_owned(), "Thermo".to_owned()],
        );
        apartament
    }

    #[test]
    fn test_constructed() {
        let apartament = build_apartament();
        assert_eq!(apartament.name, "Living room");
        assert_eq!(
            apartament.devices,
            vec!["Socket".to_owned(), "Thermo".to_owned()]
        );
    }

    #[test]
    fn test_correct_name_apartament() {
        let apartament = build_apartament();
        assert_eq!(apartament.name(), "Living room");
    }

    #[test]
    fn test_correct_name_devices() {
        let apartament = build_apartament();
        let expected = vec!["Socket".to_owned(), "Thermo".to_owned()];
        assert_eq!(apartament.devices(), &expected);
    }

    #[test]
    fn test_corrent_size_rooms() {
        let apartament = build_apartament();
        assert!(apartament.devices().len() == 2);

        let apartament = Apartament::new("Living room", vec![]);
        assert!(apartament.devices().is_empty());
    }
}
