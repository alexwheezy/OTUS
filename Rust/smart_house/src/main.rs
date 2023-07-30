#![allow(dead_code)]
#![allow(unused_variables)]

use smart_house::{
    devices::smart::{socket::Socket, thermo::Thermometer},
    house::{House, Rooms},
    providers::info::{BorrowingDeviceInfoProvider, OwningDeviceInfoProvider},
    units::physics::{Power, Temperature},
};

fn main() {
    // Инициализация устройств
    let socket1 = Socket::new("Socket1".to_owned(), Power::Watt(1350.0));
    let socket2 = Socket::new("Socket2".to_owned(), Power::Kilowatt(1.50));
    let thermo = Thermometer::new("Thermo1".to_owned(), Temperature::Celsius(32.5));

    // Инициализация дома
    let mut house = House::new("Paradise", Rooms::new());
    let room_names = ["Living room", "Bedroom", "Kids room", "Guest room"];

    for name in room_names {
        house
            .add_room(name)
            .add_device(socket1.name())
            .add_device(socket2.name())
            .add_device(thermo.name());
    }

    // Строим отчёт с использованием `OwningDeviceInfoProvider`.
    let info_provider_1 = OwningDeviceInfoProvider::new(socket1);
    // todo: после добавления обобщённого аргумента в метод, расскоментировать передачу параметра
    let report1 = house.create_report(&info_provider_1);

    // Строим отчёт с использованием `BorrowingDeviceInfoProvider`.
    let info_provider_2 = BorrowingDeviceInfoProvider::new(&socket2, &thermo);
    // todo: после добавления обобщённого аргумента в метод, расскоментировать передачу параметра
    let report2 = house.create_report(&info_provider_2);

    // Выводим отчёты на экран:
    println!("Report #1: {report1}");
    println!("Report #2: {report2}");
}
