#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::{HashMap, HashSet};

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

    let devices_living_room =
        Room::new(HashSet::from(["Socket1".to_owned(), "Socket2".to_owned()]));
    let devices_bedroom = Room::new(HashSet::from(["Socket1".to_owned(), "Thermo1".to_owned()]));
    let devices_kids_room = Room::new(HashSet::from(["Socket2".to_owned(), "Thermo1".to_owned()]));

    //Organization of premises in the house.
    let house = House::new(
        "Paradise",
        HashMap::from([
            ("Living room".to_owned(), devices_living_room),
            ("Bedroom".to_owned(), devices_bedroom),
            ("Kids room".to_owned(), devices_kids_room),
        ]),
    );

    //Create a status the device using with `OwningDeviceInfoProvider`.
    let info_provider_1 = OwningDeviceInfoProvider::new(socket1);
    println!(
        "{}",
        info_provider_1
            .status("Socket1")
            .unwrap_or("Device not found.".to_owned())
    );

    //Create a status the device using with `BorrowingDeviceInfoProvider`.
    let info_provider_2 = BorrowingDeviceInfoProvider::new(&socket2, &thermo);
    println!(
        "{}",
        info_provider_2
            .status("Thermo1")
            .unwrap_or("Device not found.".to_owned())
    );
}
