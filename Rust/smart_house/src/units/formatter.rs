use crate::units::{physics, Display, Enable};

impl Display for Enable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Enable::On => write!(f, "On"),
            Enable::Off => write!(f, "Off"),
        }
    }
}

impl Display for physics::Power {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use physics::Power::*;
        match self {
            Watt(value) => write!(f, "{value:.2}W"),
            Kilowatt(value) => write!(f, "{value:.2}kW"),
        }
    }
}

impl Display for physics::Temperature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use physics::Temperature::*;
        match self {
            Celsius(value) => write!(f, "{value:.1}°C"),
            Fahrenheit(value) => write!(f, "{value:.1}°F"),
            Kelvin(value) => write!(f, "{value:.1}°K"),
        }
    }
}

impl Display for physics::Humidity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.1}%", self.0)
    }
}
