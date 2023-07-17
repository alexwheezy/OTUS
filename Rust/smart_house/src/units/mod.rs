mod formatter;

use physics::Power::*;
use physics::Temperature::*;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Enable {
    On,
    Off,
}

pub mod physics {
    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
    pub enum Power {
        Watt(f32),
        Kilowatt(f32),
    }

    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
    pub enum Temperature {
        Celsius(f32),
        Fahrenheit(f32),
        Kelvin(f32),
    }

    #[derive(Debug, PartialEq, PartialOrd)]
    pub struct Humidity(pub f32);
}

impl physics::Power {
    const UNIT_POWER: f32 = 1000.0;

    pub fn as_killowatt(&self) -> Self {
        match self {
            Watt(value) => Kilowatt(value / Self::UNIT_POWER),
            Kilowatt(_) => *self,
        }
    }

    pub fn as_watt(&self) -> Self {
        match self {
            Kilowatt(value) => Watt(value * Self::UNIT_POWER),
            Watt(_) => *self,
        }
    }
}

impl physics::Temperature {
    pub fn as_celsius(&self) -> Self {
        match self {
            Kelvin(value) => Celsius(value - 273.0),
            Fahrenheit(value) => Celsius((value - 32.0) / 1.8),
            Celsius(_) => *self,
        }
    }

    pub fn as_fahrenheit(&self) -> Self {
        match self {
            Kelvin(value) => Fahrenheit((value - 273.0) * 1.8 + 32.0),
            Celsius(value) => Fahrenheit(value * 1.8 + 32.0),
            Fahrenheit(_) => *self,
        }
    }

    pub fn as_kelvin(&self) -> Self {
        match self {
            Fahrenheit(value) => Kelvin((value + 459.6) * 5.0 / 9.0),
            Celsius(value) => Kelvin(value + 273.0),
            Kelvin(_) => *self,
        }
    }
}
