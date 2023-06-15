#![allow(dead_code)]
#![allow(unused_variables)]

enum Enable {
    On,
    Off,
}

enum Power {
    Watt(f32),
    Kilowatt(f32),
}

enum Degrees {
    Celsius(f32),
    Fahrenheit(f32),
    Kelvin(f32),
}

struct Point {
    x: f32,
    y: f32,
    z: f32,
}

struct SmartSocket {
    //TODO: It might be worth converting it to a string slice later.
    name: String,
    position: Point,
    value: Power,
    enable: Enable,
}

impl SmartSocket {
    /// The constructor will immediately turn on the socket
    /// and initialize the initial value of the power consumption.
    fn new(init: Power) -> Self {
        todo!("Implement initial device initialization")
    }

    /// The device description contains information about its position in the room,
    /// the name of the room, and the name of the device itself.
    fn description(&self) -> String {
        todo!(
            "It is necessary to implement a
            textual description of the power consumption data"
        )
    }

    fn into(&mut self, unit: Power) {
        todo!("Implement conversion between different units for display")
    }

    fn power_consumption(&self) -> Power {
        todo!("Implement current power consumption report")
    }

    fn switch(&mut self, state: Enable) {
        todo!("Implement device on and off state")
    }
}

struct SmartThermometer {
    //TODO: It might be worth converting it to a string slice later.
    name: String,
    position: Point,
    value: Degrees,
    enable: Enable,
}

impl SmartThermometer {
    /// The constructor will immediately turn on the thermometer
    /// and initialize the initial value of the room temperature.
    fn new(init: Degrees) -> Self {
        todo!("Implement initial device initialization")
    }

    /// The device description contains information about its position in the room,
    /// the name of the room, and the name of the device itself.
    fn description(&self) -> String {
        todo!(
            "It is necessary to implement a
            textual description of the temperature and humidity in the room"
        )
    }

    fn humidity(&self) -> u8 {
        todo!("Implement indoor air humidity indicator")
    }

    fn temperature(&self) -> Degrees {
        todo!(
            "Implement the collection of data on 
            the current temperature in the room"
        )
    }

    fn into(&mut self, unit: Degrees) {
        todo!("Implement conversion between different units for display")
    }

    fn set(&mut self, new_value: Degrees) {
        todo!("Implement setting a new temperature value")
    }

    fn reset(&mut self) {
        todo!("Implement a reset of the current device settings")
    }

    fn switch(&mut self, state: Enable) {
        todo!("Implement device on and off state")
    }
}

fn main() {}
