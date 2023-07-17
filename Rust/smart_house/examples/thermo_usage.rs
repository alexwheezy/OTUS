#![allow(dead_code)]
#![allow(unused_variables)]

use smart_house::{
    devices::smart::thermo::Thermometer,
    units::{physics::Temperature, Enable},
};

fn main() {
    //This example demonstrates how we can query and control digital devices in the rooms.
    let mut thermo = Thermometer::new("Thermo".to_owned(), Temperature::Celsius(30.1));
    println!("{}", thermo.description());

    //Setting up new settings and options.
    thermo.set_temperature(Temperature::Fahrenheit(451.5));
    thermo.switch(Enable::Off);
    //Reload the device.
    thermo.reload();

    println!("{}", thermo.description());
}
