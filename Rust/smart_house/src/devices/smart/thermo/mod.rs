#![allow(unused_variables)]
#![allow(dead_code)]

use crate::units::{physics, Enable};

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

    pub fn humidity(&self) -> physics::Humidity {
        //TODO: Make a correct calculation of relative humidity.
        physics::Humidity(58.5)
    }

    pub fn temperature(&self) -> &physics::Temperature {
        &self.temperature
    }

    pub fn into(&mut self, unit: physics::Temperature) {
        todo!("Implement conversion between different units for display")
    }

    pub fn set(&mut self, value: physics::Temperature) {
        self.temperature = value;
    }

    pub fn reset(&mut self) {
        todo!("Implement a reset of the current device settings")
    }

    fn switch(&mut self, state: Enable) {
        match self.state {
            Enable::On => self.state = Enable::Off,
            Enable::Off => self.state = Enable::On,
        }
    }
}
