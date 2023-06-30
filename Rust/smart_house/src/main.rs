#![allow(dead_code)]
#![allow(unused_variables)]

use std::fmt::Display;

enum Enable {
    On,
    Off,
}

enum Power {
    Watt(f32),
    Kilowatt(f32),
}

enum Temperature {
    Celsius(f32),
    Fahrenheit(f32),
    Kelvin(f32),
}

struct SmartSocket {
    name: String,
    power: Power,
    state: Enable,
}

struct Humidity(f32);

impl Display for Enable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Enable::On => write!(f, "On"),
            Enable::Off => write!(f, "Off"),
        }
    }
}

impl Display for Power {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Power::Watt(value) => write!(f, "{value:.2}W"),
            Power::Kilowatt(value) => write!(f, "{value:.2}kW"),
        }
    }
}
impl Display for Temperature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let spec_sym = "\u{00b0}";
        match self {
            Temperature::Celsius(value) => write!(f, "{value:.1}{spec_sym}C"),
            Temperature::Fahrenheit(value) => write!(f, "{value:.1}{spec_sym}F"),
            Temperature::Kelvin(value) => write!(f, "{value:.1}{spec_sym}K"),
        }
    }
}

impl Display for Humidity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\u{0025}", self.0)
    }
}

// ***** Пример библиотеки "Умный дом" со статическим содержимым

#[derive(Debug, Clone)]
struct Room {
    name: String,
    devices: Vec<String>,
}

impl Room {
    fn new(name: &str, devices: Vec<String>) -> Self {
        Self {
            name: name.to_owned(),
            devices,
        }
    }
}

#[derive(Debug, Clone)]
struct SmartHouse {
    name: String,
    rooms: Vec<Room>,
}

impl SmartHouse {
    fn new(name: &str, rooms: Vec<Room>) -> Self {
        Self {
            name: name.to_owned(),
            rooms,
        }
    }

    ///Return list of rooms in the house.
    fn get_rooms(&self) -> Vec<String> {
        self.rooms
            .iter()
            .map(|current_room| current_room.name.to_owned())
            .collect()
    }

    ///Return list of devices in the room.
    fn devices(&self, room: &str) -> Vec<String> {
        self.rooms
            .iter()
            .filter(|&current_room| current_room.name == room)
            .flat_map(|room| room.devices.clone())
            .collect()
    }

    ///Text report on the status of devices in the house.
    fn create_report(&self, provider: &impl DeviceInfoProvider) -> String {
        //Report header
        let mut report = format!("\n{:>12}: [{}]\n", "House", &self.name);

        for room in self.get_rooms() {
            report.push_str(&format!("\n{:>12}: [{}]", "Room", room));
            for device in self.devices(&room) {
                report.push_str(&provider.status(&device));
            }
        }
        report
    }
}

impl SmartSocket {
    /// The constructor will immediately turn on the socket
    /// and initialize the initial value of the power consumption.
    fn new(name: String, init: Power) -> Self {
        Self {
            name,
            power: init,
            state: Enable::On,
        }
    }

    /// The device description contains information about its position in the room,
    /// the name of the room, and the name of the device itself.
    fn description(&self) -> String {
        let mut summmary = String::with_capacity(3);
        summmary.push('\n');
        summmary.push_str(&format!("{:>12}: {}\n", "Device", self.name));
        summmary.push_str(&format!("{:>12}: {}\n", "Power", self.power_consumption()));
        summmary.push_str(&format!("{:>12}: {}\n", "State", self.state));
        summmary
    }

    fn into(&mut self, unit: Power) {
        todo!("Implement conversion between different units for display")
    }

    fn power_consumption(&self) -> &Power {
        &self.power
    }

    fn switch(&mut self, state: Enable) {
        match self.state {
            Enable::On => self.state = Enable::Off,
            Enable::Off => self.state = Enable::On,
        }
    }
}

struct SmartThermometer {
    name: String,
    temperature: Temperature,
    state: Enable,
}

impl SmartThermometer {
    /// The constructor will immediately turn on the thermometer
    /// and initialize the initial value of the room temperature.
    fn new(name: String, init: Temperature) -> Self {
        Self {
            name,
            temperature: init,
            state: Enable::On,
        }
    }

    /// The device description contains information about its position in the room,
    /// the name of the room, and the name of the device itself.
    fn description(&self) -> String {
        let mut summmary = String::with_capacity(4);
        summmary.push('\n');
        summmary.push_str(&format!("{:>12}: {}\n", "Device", self.name));
        summmary.push_str(&format!("{:>12}: {}\n", "Temperature", self.temperature()));
        summmary.push_str(&format!("{:>12}: {}\n", "Humidity", self.humidity()));
        summmary.push_str(&format!("{:>12}: {}\n", "State", self.state));
        summmary
    }

    fn humidity(&self) -> Humidity {
        //TODO: Make a correct calculation of relative humidity.
        Humidity(58.5)
    }

    fn temperature(&self) -> &Temperature {
        &self.temperature
    }

    fn into(&mut self, unit: Temperature) {
        todo!("Implement conversion between different units for display")
    }

    fn set(&mut self, value: Temperature) {
        self.temperature = value;
    }

    fn reset(&mut self) {
        todo!("Implement a reset of the current device settings")
    }

    fn switch(&mut self, state: Enable) {
        match self.state {
            Enable::On => self.state = Enable::Off,
            Enable::Off => self.state = Enable::On,
        }
    }
}

trait DeviceInfoProvider {
    fn status(&self, device: &str) -> String;
}

// Пользовательские поставщики информации об устройствах.
// Могут как хранить устройства, так и заимствывать.
struct OwningDeviceInfoProvider {
    socket: SmartSocket,
}
struct BorrowingDeviceInfoProvider<'a, 'b> {
    socket: &'a SmartSocket,
    thermo: &'b SmartThermometer,
}

impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn status(&self, device: &str) -> String {
        match self.socket.name == device {
            true => self.socket.description(),
            false => format!("\nError! Device {} not found.\n", device),
        }
    }
}

impl<'a, 'b> DeviceInfoProvider for BorrowingDeviceInfoProvider<'a, 'b> {
    fn status(&self, device: &str) -> String {
        if self.socket.name == device {
            self.socket.description()
        } else if self.thermo.name == device {
            self.thermo.description()
        } else {
            format!("\nError! Device {} not found.\n", device)
        }
    }
}

// ***** Пример использования библиотеки умный дом:
fn main() {
    // Инициализация устройств
    let socket1 = SmartSocket::new("Socket1".to_owned(), Power::Watt(1350.0));
    let socket2 = SmartSocket::new("Socket2".to_owned(), Power::Kilowatt(1.50));
    let thermo = SmartThermometer::new("Thermo1".to_owned(), Temperature::Celsius(32.5));

    // Инициализация дома
    #[rustfmt::skip]
    let house = SmartHouse::new(
        "Paradise",
        vec![
            Room::new(
                "Living room",
                vec!["Socket1".to_owned(), "Socket2".to_owned()],
            ),
            Room::new(
                "Bedroom", 
                vec!["Socket1".to_owned(), "Thermo1".to_owned()]
            ),
            Room::new(
                "Kids room",
                vec!["Socket2".to_owned(), "Thermo1".to_owned()],
            ),
        ],
    );

    // Строим отчёт с использованием `OwningDeviceInfoProvider`.
    let info_provider_1 = OwningDeviceInfoProvider { socket: socket1 };
    // todo: после добавления обобщённого аргумента в метод, расскоментировать передачу параметра
    let report1 = house.create_report(&info_provider_1);

    // Строим отчёт с использованием `BorrowingDeviceInfoProvider`.
    let info_provider_2 = BorrowingDeviceInfoProvider {
        socket: &socket2,
        thermo: &thermo,
    };
    // todo: после добавления обобщённого аргумента в метод, расскоментировать передачу параметра
    let report2 = house.create_report(&info_provider_2);

    // Выводим отчёты на экран:
    println!("Report #1: {report1}");
    println!("Report #2: {report2}");
}
