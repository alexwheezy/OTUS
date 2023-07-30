#![allow(dead_code)]
#![allow(unused_variables)]

use smart_house::{
    devices::smart::socket::Socket,
    house::{House, Rooms},
    providers::info::OwningDeviceInfoProvider,
    units::physics::Power,
};

fn main() {
    // Инициализация устройств
    let socket1 = Socket::new("Socket1".to_owned(), Power::Watt(1350.0));
    let socket2 = Socket::new("Socket2".to_owned(), Power::Watt(1120.0));

    // Инициализация дома
    let mut house = House::new("Paradise", Rooms::new());

    house
        .add_room("Kids room")
        .add_device(socket1.name())
        .add_device(socket2.name());

    house.add_room("Guest room").add_device(socket1.name());

    // Строим отчёт с использованием `OwningDeviceInfoProvider`.
    let info_provider_1 = OwningDeviceInfoProvider::new(socket1);
    let info_provider_2 = OwningDeviceInfoProvider::new(socket2);

    let report1 = house.create_report(&info_provider_1);
    let report2 = house.create_report(&info_provider_2);

    // Выводим отчёты на экран:
    println!("Report #1: {report1}");
    println!("Report #2: {report2}");
}
