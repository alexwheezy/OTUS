use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Enable {
    On,
    Off,
}

impl Display for Enable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Enable::On => write!(f, "On"),
            Enable::Off => write!(f, "Off"),
        }
    }
}

pub mod physics {
    #[derive(Debug, PartialEq)]
    pub enum Power {
        Watt(f32),
        Kilowatt(f32),
    }

    #[derive(Debug, PartialEq)]
    pub enum Temperature {
        Celsius(f32),
        Fahrenheit(f32),
        Kelvin(f32),
    }

    #[derive(Debug, PartialEq)]
    pub struct Humidity(pub f32);
}

impl Display for physics::Power {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            physics::Power::Watt(value) => write!(f, "{value:.2}W"),
            physics::Power::Kilowatt(value) => write!(f, "{value:.2}kW"),
        }
    }
}

impl Display for physics::Temperature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let spec_sym = "\u{00b0}";
        match self {
            physics::Temperature::Celsius(value) => write!(f, "{value:.1}{spec_sym}C"),
            physics::Temperature::Fahrenheit(value) => write!(f, "{value:.1}{spec_sym}F"),
            physics::Temperature::Kelvin(value) => write!(f, "{value:.1}{spec_sym}K"),
        }
    }
}

impl Display for physics::Humidity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\u{0025}", self.0)
    }
}
