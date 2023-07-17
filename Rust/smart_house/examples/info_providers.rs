#![allow(dead_code)]
#![allow(unused_variables)]

use smart_house::{
    devices::smart::{socket::Socket, thermo::Thermometer},
    house::{room::Room, House},
    providers::info::{BorrowingDeviceInfoProvider, DeviceInfoProvider, OwningDeviceInfoProvider},
    units::physics::{Power, Temperature},
};

fn main() {
    //Initialization of used devices.
    let socket1 = Socket::new("Socket1".to_owned(), Power::Watt(1350.0));
    let socket2 = Socket::new("Socket2".to_owned(), Power::Kilowatt(1.50));
    let thermo = Thermometer::new("Thermo1".to_owned(), Temperature::Celsius(32.5));

    //Organization of premises in the house.
    #[rustfmt::skip]
    let house = House::new(
        "Paradise",
        vec![
            Room::new(
                "Living room",
                vec!["Socket1".to_owned(), "Socket2".to_owned()],
            ),
            Room::new(
                "Bedroom", 
                vec!["Socket1".to_owned(), "Thermo1".to_owned()],
            ),
            Room::new(
                "Kids room",
                vec!["Socket2".to_owned(), "Thermo1".to_owned()],
            ),
        ],
    );

    //Create a status the device using with `OwningDeviceInfoProvider`.
    let info_provider_1 = OwningDeviceInfoProvider::new(socket1);
    println!(
        "Info provider: {}",
        info_provider_1
            .status("Socket1")
            .unwrap_or("Device not found.".to_owned())
    );

    //Create a status the device using with `BorrowingDeviceInfoProvider`.
    let info_provider_2 = BorrowingDeviceInfoProvider::new(&socket2, &thermo);
    println!(
        "Info provider: {}",
        info_provider_2
            .status("Thermo1")
            .unwrap_or("Device not found.".to_owned())
    );
}
