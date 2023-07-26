#![allow(unused_variables)]
#![allow(dead_code)]

use crate::units::{
    physics::{
        Humidity,
        Temperature::{self, Celsius, Fahrenheit, Kelvin},
    },
    Enable,
};

#[derive(Debug, PartialEq)]
pub struct Thermometer {
    //Device name, can be anything.
    name: String,
    //The current room temperature indicator.
    temperature: Temperature,
    //Relative humidity in the room.
    humidity: Humidity,
    state: Enable,
}

impl Default for Thermometer {
    fn default() -> Self {
        Self {
            name: "Thermometer".to_owned(),
            temperature: Temperature::Celsius(0.0),
            humidity: Humidity(0.0),
            state: Enable::Off,
        }
    }
}

impl Thermometer {
    ///The constructor will immediately turn on the thermometer
    ///and initialize the initial value of the room temperature.
    pub fn new(name: String, init: Temperature) -> Self {
        //NOTE: Very rough calculation of relative humidity in a room.
        //Do I need to build a dependency table?
        let p = 0.012;
        let p0 = match init.as_celsius() {
            Celsius(value) => value / 1000.0,
            _ => unreachable!(),
        };
        let humidity = Humidity((p / p0) * 100.0);

        Self {
            name,
            temperature: Self::clamp_temeperate(init),
            humidity,
            state: Enable::On,
        }
    }

    ///The device description contains information about its position in the room,
    ///the name of the room, and the name of the device itself.
    pub fn description(&self) -> String {
        format!(
            r#"
      Device: {name}
 Temperature: {temperature}
    Humidity: {humidity}
       State: {state}"#,
            name = self.name,
            temperature = self.temperature(),
            humidity = self.humidity(),
            state = self.state,
        )
    }

    ///Return the device name.
    pub fn name(&self) -> &str {
        &self.name
    }

    ///Return the current humidity in the apartement.
    pub fn humidity(&self) -> &Humidity {
        &self.humidity
    }

    ///The current room temperature received from the device.
    pub fn temperature(&self) -> &Temperature {
        &self.temperature
    }

    pub(crate) fn promote_units(&mut self) {
        let units = match self.temperature() {
            Celsius(_) => Temperature::as_fahrenheit(self.temperature()),
            Fahrenheit(_) => Temperature::as_kelvin(self.temperature()),
            Kelvin(_) => Temperature::as_celsius(self.temperature()),
        };
        self.temperature = units;
    }

    ///Setting a new temperature value.
    pub fn set_temperature(&mut self, value: Temperature) {
        self.temperature = Self::clamp_temeperate(value);
    }

    pub(crate) fn clamp_temeperate(value: Temperature) -> Temperature {
        match value {
            Celsius(data) => Celsius(data.clamp(-100.0, 100.0)),
            Fahrenheit(data) => Fahrenheit(data.clamp(-500.0, 500.0)),
            Kelvin(data) => Kelvin(data.clamp(-250.0, 250.0)),
        }
    }

    ///Reload the current device.
    ///A reboot resets the values in the device.
    pub fn reload(&mut self) {
        let default = Self::default();
        self.temperature = default.temperature;
        self.humidity = default.humidity;
        self.state = default.state;
    }

    pub fn switch(&mut self, state: Enable) {
        match self.state {
            Enable::On => self.state = Enable::Off,
            Enable::Off => self.state = Enable::On,
        }
        match self.temperature {
            Celsius(_) => self.temperature = Celsius(0.0),
            Fahrenheit(_) => self.temperature = Fahrenheit(0.0),
            Kelvin(_) => self.temperature = Kelvin(0.0),
        }
        self.humidity = Humidity(0.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correct_device_name() {
        let thermo = Thermometer::new("Default".to_owned(), Temperature::Celsius(29.5));
        let expected = "Default";
        assert_eq!(thermo.name(), expected);
    }

    #[test]
    fn test_correct_temperature() {
        let thermo = Thermometer::new("Thermometer".to_owned(), Temperature::Celsius(29.5));
        let expected = Temperature::Celsius(29.5);
        assert!(thermo.temperature().le(&expected));
    }

    #[test]
    fn test_set_temperature() {
        let mut thermo = Thermometer::new("Thermometer".to_owned(), Temperature::Celsius(29.5));
        thermo.set_temperature(Temperature::Celsius(20.6));
        assert!(thermo.temperature().le(&Temperature::Celsius(20.6)));
    }

    #[test]
    fn test_as_fahrenheit() {
        let mut thermo = Thermometer::new("Thermometer".to_owned(), Temperature::Celsius(29.5));
        thermo.promote_units();
        assert!(thermo.temperature().le(&Temperature::Fahrenheit(85.1)));
    }

    #[test]
    fn test_as_kelvin() {
        let mut thermo = Thermometer::new("Thermometer".to_owned(), Temperature::Fahrenheit(81.5));
        thermo.promote_units();
        assert!(thermo.temperature().le(&Temperature::Kelvin(302.5)));
    }

    #[test]
    fn test_as_celsius() {
        let mut thermo = Thermometer::new("Thermometer".to_owned(), Temperature::Kelvin(302.5));
        thermo.promote_units();
        assert!(thermo.temperature().le(&Temperature::Celsius(29.5)));
    }

    #[test]
    fn test_promote_units() {
        let mut thermo = Thermometer::new("Thermometer".to_owned(), Temperature::Celsius(29.5));
        thermo.promote_units();
        thermo.promote_units();
        thermo.promote_units();
        assert!(thermo.temperature().ge(&Temperature::Celsius(29.5)));
    }

    #[test]
    fn test_reload_device() {
        let mut thermo = Thermometer::new("Thermometer".to_owned(), Temperature::Fahrenheit(451.1));
        thermo.reload();
        assert!(thermo.temperature().le(&Temperature::Celsius(0.0)));
        assert!(thermo.humidity().le(&Humidity(0.0)));
        assert!(thermo.state == Enable::Off);
    }

    #[test]
    fn test_switch_state() {
        let mut thermo = Thermometer::new("Thermometer".to_owned(), Temperature::Celsius(29.5));
        assert_eq!(thermo.state, Enable::On);

        thermo.switch(Enable::Off);
        assert_eq!(thermo.state, Enable::Off);
    }

    #[test]
    fn test_correct_description() {
        let thermo = Thermometer::new("Thermometer".to_owned(), Temperature::Celsius(29.5));
        let expected = format!(
            r#"
      Device: Thermometer
 Temperature: 29.5°C
    Humidity: 40.7%
       State: On"#,
        );
        assert_eq!(thermo.description(), expected);
    }
}
