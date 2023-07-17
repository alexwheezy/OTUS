#![allow(unused_variables)]
#![allow(dead_code)]

use crate::units::{
    physics::Power::{self, Kilowatt, Watt},
    Enable,
};

#[derive(Debug, PartialEq)]
pub struct Socket {
    //Device name, can be anything.
    name: String,
    //Power consumption of the device.
    power: Power,
    state: Enable,
}

impl Default for Socket {
    fn default() -> Self {
        Self {
            name: "Socket".to_owned(),
            power: Power::Watt(1350.0),
            state: Enable::On,
        }
    }
}

impl Socket {
    ///The constructor will immediately turn on the socket
    ///and initialize the initial value of the power consumption.
    pub fn new(name: String, init: Power) -> Self {
        assert!(!name.is_empty(), "The device must have a name.");
        Self {
            name,
            power: Self::clamp_power(init),
            state: Enable::On,
        }
    }

    ///The device description contains information about its position in the room,
    ///the name of the room, and the name of the device itself.
    pub fn description(&self) -> String {
        let mut summmary = String::with_capacity(64);
        summmary.push('\n');
        summmary.push_str(&format!("{:>12}: {}\n", "Device", self.name));
        summmary.push_str(&format!("{:>12}: {}\n", "Power", self.power_consumption()));
        summmary.push_str(&format!("{:>12}: {}\n", "State", self.state));
        summmary
    }

    ///Return device the name.
    pub fn name(&self) -> &str {
        &self.name
    }

    ///The method call will advance new units each time the measurement is made around the circle.
    pub(crate) fn promote_units(&mut self) {
        let units = match self.power_consumption() {
            Watt(_) => Power::as_killowatt(&self.power),
            Kilowatt(_) => Power::as_watt(&self.power),
        };
        self.power = units;
    }

    ///Return power_consumption the current device.
    pub fn power_consumption(&self) -> &Power {
        &self.power
    }

    ///Setting a new power value. The minimum and maximum value of the capped limit.
    pub fn set_power(&mut self, value: Power) {
        self.power = Self::clamp_power(value);
    }

    pub(crate) fn clamp_power(value: Power) -> Power {
        match value {
            Watt(data) => Watt(data.clamp(0.0, 2500.0)),
            Kilowatt(data) => Kilowatt(data.clamp(0.0, 2.5)),
        }
    }

    ///Switching the state of the device turns it off and resets the power value to zero.
    ///After switching on, the power value must be updated.
    pub fn switch(&mut self, state: Enable) {
        match self.state {
            Enable::On => self.state = Enable::Off,
            Enable::Off => self.state = Enable::On,
        }
        match self.power {
            Watt(_) => self.power = Watt(0.0),
            Kilowatt(_) => self.power = Kilowatt(0.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correct_device_name() {
        let socket = Socket::new("Default".to_owned(), Power::Watt(1300.0));
        let expected = "Default";
        assert_eq!(socket.name(), expected);
    }

    #[test]
    fn test_correct_power_consumption() {
        let socket = Socket::new("Socket".to_owned(), Power::Watt(1200.0));
        let expected = Power::Watt(1200.0);
        assert!(socket.power_consumption().le(&expected));
    }

    #[test]
    fn test_switch_state() {
        let mut socket = Socket::new("Socket".to_owned(), Power::Watt(1500.0));
        assert_eq!(socket.state, Enable::On);

        socket.switch(Enable::Off);
        assert_eq!(socket.state, Enable::Off);
        assert!(socket.power_consumption().le(&Power::Watt(0.0)));
    }

    #[test]
    fn test_correct_description() {
        let socket = Socket::new("Socket".to_owned(), Power::Watt(1510.0));

        let mut expected = String::with_capacity(5);
        expected.push('\n');
        expected.push_str(&format!("{:>12}: {}\n", "Device", "Socket"));
        expected.push_str(&format!("{:>12}: {}\n", "Power", "1510.00W"));
        expected.push_str(&format!("{:>12}: {}\n", "State", "On"));

        assert_eq!(socket.description(), expected);
    }

    #[test]
    fn test_promote_units() {
        let mut socket = Socket::new("Socket".to_owned(), Power::Kilowatt(1.51));
        socket.promote_units();
        assert!(socket.power_consumption().le(&Power::Watt(1510.0)));

        let mut socket = Socket::new("Socket".to_owned(), Power::Watt(1510.0));
        socket.promote_units();
        socket.promote_units();
        assert!(socket.power_consumption().le(&Power::Watt(1510.0)));
    }

    #[test]
    fn test_set_power_value() {
        let mut socket = Socket::new("Socket".to_owned(), Power::Watt(1510.0));
        socket.set_power(Power::Kilowatt(1.35));
        assert!(socket.power_consumption().le(&Power::Kilowatt(1.35)));
    }

    #[test]
    fn test_power_max_value() {
        let mut socket = Socket::new("Socket".to_owned(), Power::Watt(1510.0));
        socket.set_power(Power::Kilowatt(42.5));
        assert!(socket.power_consumption().le(&Power::Kilowatt(2.5)));

        socket.set_power(Power::Watt(10000.0));
        assert!(socket.power_consumption().le(&Power::Kilowatt(2500.0)));
    }

    #[test]
    fn test_power_min_value() {
        let mut socket = Socket::new("Socket".to_owned(), Power::Watt(1510.0));
        socket.set_power(Power::Kilowatt(-1.5));
        assert!(socket.power_consumption().le(&Power::Kilowatt(0.0)));

        socket.set_power(Power::Watt(-1000.0));
        assert!(socket.power_consumption().le(&Power::Watt(0.0)));
    }
}
