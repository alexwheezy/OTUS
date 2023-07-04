#![allow(unused_variables)]
#![allow(dead_code)]

use crate::units::{physics, Enable};

pub struct Socket {
    name: String,
    power: physics::Power,
    state: Enable,
}

impl Socket {
    /// The constructor will immediately turn on the socket
    /// and initialize the initial value of the power consumption.
    pub fn new(name: String, init: physics::Power) -> Self {
        Self {
            name,
            power: init,
            state: Enable::On,
        }
    }

    /// The device description contains information about its position in the room,
    /// the name of the room, and the name of the device itself.
    pub fn description(&self) -> String {
        let mut summmary = String::with_capacity(3);
        summmary.push('\n');
        summmary.push_str(&format!("{:>12}: {}\n", "Device", self.name));
        summmary.push_str(&format!("{:>12}: {}\n", "Power", self.power_consumption()));
        summmary.push_str(&format!("{:>12}: {}\n", "State", self.state));
        summmary
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn into(&mut self, unit: physics::Power) {
        todo!("Implement conversion between different units for display")
    }

    pub fn power_consumption(&self) -> &physics::Power {
        &self.power
    }

    pub fn switch(&mut self, state: Enable) {
        match self.state {
            Enable::On => self.state = Enable::Off,
            Enable::Off => self.state = Enable::On,
        }
    }
}
