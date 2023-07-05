#![allow(unused_variables)]
#![allow(dead_code)]

use crate::units::{physics, Enable};

#[derive(Debug, PartialEq)]
pub struct Thermometer {
    name: String,
    temperature: physics::Temperature,
    state: Enable,
}

impl Thermometer {
    /// The constructor will immediately turn on the thermometer
    /// and initialize the initial value of the room temperature.
    pub fn new(name: String, init: physics::Temperature) -> Self {
        Self {
            name,
            temperature: init,
            state: Enable::On,
        }
    }

    /// The device description contains information about its position in the room,
    /// the name of the room, and the name of the device itself.
    pub fn description(&self) -> String {
        let mut summmary = String::with_capacity(4);
        summmary.push('\n');
        summmary.push_str(&format!("{:>12}: {}\n", "Device", self.name));
        summmary.push_str(&format!("{:>12}: {}\n", "Temperature", self.temperature()));
        summmary.push_str(&format!("{:>12}: {}\n", "Humidity", self.humidity()));
        summmary.push_str(&format!("{:>12}: {}\n", "State", self.state));
        summmary
    }

    /// The device name
    pub fn name(&self) -> &str {
        &self.name
    }

    fn humidity(&self) -> physics::Humidity {
        //TODO: Make a correct calculation of relative humidity.
        physics::Humidity(58.5)
    }

    pub fn temperature(&self) -> &physics::Temperature {
        &self.temperature
    }

    fn into(&mut self, unit: physics::Temperature) {
        todo!("Implement conversion between different units for display")
    }

    pub fn set(&mut self, value: physics::Temperature) {
        self.temperature = value;
    }

    fn reset(&mut self) {
        todo!("Implement a reset of the current device settings")
    }

    pub fn switch(&mut self, state: Enable) {
        match self.state {
            Enable::On => self.state = Enable::Off,
            Enable::Off => self.state = Enable::On,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructed() {
        let thermo = Thermometer::new(
            "Thermometer".to_owned(),
            physics::Temperature::Celsius(29.5),
        );
        assert_eq!(
            thermo,
            Thermometer {
                name: "Thermometer".to_owned(),
                temperature: physics::Temperature::Celsius(29.5),
                state: Enable::On,
            }
        );
    }

    #[test]
    fn test_correct_name() {
        let thermo = Thermometer::new("Default".to_owned(), physics::Temperature::Celsius(29.5));
        let expected = "Default";
        assert_eq!(thermo.name(), expected);
    }

    #[test]
    fn test_correct_temperature() {
        let thermo = Thermometer::new(
            "Thermometer".to_owned(),
            physics::Temperature::Celsius(29.5),
        );
        let expected = physics::Temperature::Celsius(29.5);
        assert!(thermo.temperature() == &expected);
    }

    #[test]
    fn test_switch_state() {
        let mut thermo = Thermometer::new(
            "Thermometer".to_owned(),
            physics::Temperature::Celsius(29.5),
        );
        assert_eq!(thermo.state, Enable::On);

        thermo.switch(Enable::Off);
        assert_eq!(thermo.state, Enable::Off);
    }

    #[test]
    fn test_correct_description() {
        let thermo = Thermometer::new(
            "Thermometer".to_owned(),
            physics::Temperature::Celsius(29.5),
        );

        let mut expected = String::with_capacity(5);
        expected.push('\n');

        expected.push_str(&format!("{:>12}: {}\n", "Device", "Thermometer"));
        expected.push_str(&format!("{:>12}: {}\n", "Temperature", "29.5\u{00b0}C"));
        expected.push_str(&format!("{:>12}: {}\n", "Humidity", "58.5\u{0025}"));
        expected.push_str(&format!("{:>12}: {}\n", "State", "On"));

        assert_eq!(thermo.description(), expected);
    }

    #[test]
    fn test_set_temperature() {
        let mut thermo = Thermometer::new(
            "Thermometer".to_owned(),
            physics::Temperature::Celsius(29.5),
        );
        thermo.set(physics::Temperature::Celsius(20.6));
        assert_eq!(thermo.temperature(), &physics::Temperature::Celsius(20.6));
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_converion_units() {
        todo!()
    }
}
