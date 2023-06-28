#![allow(dead_code)]
#![allow(unused_variables)]

struct SmartHouse {/* todo: данные умного дома */}

impl SmartHouse {
    fn new() -> Self {
        todo!("реализовать инициализацию дома")
    }

    fn get_rooms(&self) -> [&str; 2] {
        // Размер возвращаемого массива можно выбрать самостоятельно
        todo!("список комнат")
    }

    fn devices(&self, room: &str) -> [&str; 3] {
        // Размер возвращаемого массива можно выбрать самостоятельно
        todo!("список устройств в комнате `room`")
    }

    fn create_report<T: DeviceInfoProvider>(&self, device: &T) -> String {
        todo!("перебор комнат и устройств в них для составления отчёта")
    }
}

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

struct Vec3d {
    x: f32,
    y: f32,
    z: f32,
}

// Пользовательские устройства:
struct SmartSocket {
    //TODO: It might be worth converting it to a string slice later.
    name: String,
    position: Vec3d,
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
    position: Vec3d,
    value: Temperature,
    enable: Enable,
}

impl SmartThermometer {
    /// The constructor will immediately turn on the thermometer
    /// and initialize the initial value of the room temperature.
    fn new(init: Temperature) -> Self {
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

    fn temperature(&self) -> Temperature {
        todo!(
            "Implement the collection of data on 
            the current temperature in the room"
        )
    }

    fn into(&mut self, unit: Temperature) {
        todo!("Implement conversion between different units for display")
    }

    fn set(&mut self, new_value: Temperature) {
        todo!("Implement setting a new temperature value")
    }

    fn reset(&mut self) {
        todo!("Implement a reset of the current device settings")
    }

    fn switch(&mut self, state: Enable) {
        todo!("Implement device on and off state")
    }
}

trait DeviceInfoProvider {}

// Пользовательские поставщики информации об устройствах.
// Могут как хранить устройства, так и заимствывать.
struct OwningDeviceInfoProvider {
    socket: SmartSocket,
}

struct BorrowingDeviceInfoProvider<'a, 'b> {
    socket: &'a SmartSocket,
    thermo: &'b SmartThermometer,
}

impl DeviceInfoProvider for OwningDeviceInfoProvider {}
impl<'a, 'b> DeviceInfoProvider for BorrowingDeviceInfoProvider<'a, 'b> {}

fn main() {
    // Инициализация устройств
    let socket1 = SmartSocket {
        name: String::from("Xiaomi"),
        position: Vec3d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        value: Power::Watt(10.0),
        enable: Enable::On,
    };
    let socket2 = SmartSocket {
        name: String::from("Xiaomi"),
        position: Vec3d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        value: Power::Watt(10.0),
        enable: Enable::On,
    };
    let thermo = SmartThermometer {
        name: String::from("Xiaomi"),
        position: Vec3d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        value: Temperature::Celsius(23.5),
        enable: Enable::On,
    };

    // Инициализация дома
    let house = SmartHouse::new();

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
