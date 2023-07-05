#![allow(dead_code)]
#![allow(unused_variables)]

use smart_house::{
    devices::smart::{socket::Socket, thermo::Thermometer},
    units::{
        physics::{Power, Temperature},
        Enable,
    },
};

fn main() {
    let mut socket = Socket::new("Socket".to_owned(), Power::Watt(1350.0));
    //Disable the device.
    socket.switch(Enable::Off);

    //Print the power consumption socket.
    println!("Socket power: {}", socket.power_consumption());
    //Prints a text description of the devices.
    println!("{}", socket.description());

    let mut thermo = Thermometer::new("Thermo".to_owned(), Temperature::Celsius(30.1));

    //Print the temperature in the apartament.
    println!("Temperature: {}", thermo.temperature());

    //Prints a text description of the devices.
    println!("{}", thermo.description());

    //Set new temperature value.
    thermo.set(Temperature::Fahrenheit(451.5));

    //Prints a text description of the devices.
    println!("{}", thermo.description());
}
