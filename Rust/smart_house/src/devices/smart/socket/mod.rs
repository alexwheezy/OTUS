#![allow(unused_variables)]
#![allow(dead_code)]

use crate::units::{physics, Enable};

#[derive(Debug, PartialEq)]
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
        let mut summmary = String::with_capacity(5);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructed() {
        let socket = Socket::new("Socket".to_owned(), physics::Power::Watt(0.35));
        assert_eq!(
            socket,
            Socket {
                name: "Socket".to_owned(),
                power: physics::Power::Watt(0.35),
                state: Enable::On,
            }
        );
    }

    #[test]
    fn test_correct_name() {
        let socket = Socket::new("Default".to_owned(), physics::Power::Watt(0.35));
        let expected = "Default";
        assert_eq!(socket.name(), expected);
    }

    #[test]
    fn test_correct_power_consumption() {
        let socket = Socket::new("Socket".to_owned(), physics::Power::Watt(0.25));
        let expected = physics::Power::Watt(0.25);
        assert!(socket.power_consumption() == &expected);
    }

    #[test]
    fn test_switch_state() {
        let mut socket = Socket::new("Socket".to_owned(), physics::Power::Watt(0.40));
        assert_eq!(socket.state, Enable::On);

        socket.switch(Enable::Off);
        assert_eq!(socket.state, Enable::Off);
    }

    #[test]
    fn test_correct_description() {
        let socket = Socket::new("Socket".to_owned(), physics::Power::Watt(0.41));

        let mut expected = String::with_capacity(5);
        expected.push('\n');
        expected.push_str(&format!("{:>12}: {}\n", "Device", "Socket"));
        expected.push_str(&format!("{:>12}: {}\n", "Power", "0.41W"));
        expected.push_str(&format!("{:>12}: {}\n", "State", "On"));

        assert_eq!(socket.description(), expected);
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_converion_units() {
        todo!()
    }
}
