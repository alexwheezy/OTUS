#![allow(dead_code)]
#![allow(unused_variables)]

use smart_house::units::physics::{Power, Temperature};

fn main() {
    //This example demonstrates how you can convert physical scalar units between the same type.
    //For example, from watts to kilowatts, from degrees Celsius to degrees Fahrenheit.

    let power = Power::Watt(1350.0);

    println!("{:>13}", "Power units:");
    println!("{:>12}: {}", "Watt", power.as_watt());
    println!("{:>12}: {}", "Killowatt", power.as_killowatt());

    let temperature = Temperature::Celsius(29.5);

    println!("\nTemperature units:");
    println!("{:>12}: {}", "Fahrenheit", temperature.as_fahrenheit());
    println!("{:>12}: {}", "Kelvin", temperature.as_kelvin());
    println!("{:>12}: {}", "Celsius", temperature.as_celsius());
}
