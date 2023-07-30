#![allow(dead_code)]
#![allow(unused_variables)]

use smart_house::units::physics::{Power, Temperature};

fn main() {
    //This example demonstrates how you can convert physical scalar units between the same type.
    //For example, from watts to kilowatts, from degrees Celsius to degrees Fahrenheit.
    let power = Power::Watt(1350.0);
    let power_units = format!(
        r#"
 Power units:
        Watt: {watt}
   Killowatt: {killowatt}"#,
        watt = power.as_watt(),
        killowatt = power.as_killowatt()
    );
    println!("{power_units}");

    let temperature = Temperature::Celsius(29.5);
    let temp_units = format!(
        r#"
  Temp units:
  Fahrenheit: {fahrenheit}
      Kelvin: {kelvin}
     Celsius: {celsius}
      "#,
        fahrenheit = temperature.as_fahrenheit(),
        kelvin = temperature.as_kelvin(),
        celsius = temperature.as_celsius(),
    );
    println!("{temp_units}");
}
