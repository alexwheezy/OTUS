use std::collections::{HashMap, HashSet};

use smart_house::{
    devices::smart::{socket::Socket, thermo::Thermometer},
    house::{room::Room, House},
    providers::info::*,
    units::physics::*,
};

fn initialize_house() -> House {
    let initialize_devices = Room::new(HashSet::from(["Socket".to_owned(), "Thermo".to_owned()]));
    House::new(
        "Paradise",
        HashMap::from([("Living room".to_owned(), initialize_devices)]),
    )
}

fn devices() -> (Socket, Thermometer) {
    (
        Socket::new("Socket".to_owned(), Power::Watt(1230.0)),
        Thermometer::new("Thermo".to_owned(), Temperature::Celsius(30.1)),
    )
}

#[test]
fn test_borrowing_status() {
    let (socket, thermo) = devices();
    let borrowing_provider = BorrowingDeviceInfoProvider::new(&socket, &thermo);
    assert_eq!(
        borrowing_provider.status("Socket").unwrap(),
        socket.description()
    );
    assert_eq!(
        borrowing_provider.status("Thermo").unwrap(),
        thermo.description()
    );
}

#[test]
fn test_owning_status() {
    let (socket, _) = devices();
    let owning_provider = OwningDeviceInfoProvider::new(socket);
    assert_eq!(
        owning_provider.status("Socket").unwrap(),
        Socket::new("Socket".to_owned(), Power::Watt(1230.0)).description()
    );
}

#[test]
#[ignore = "result output is non-deterministic"]
fn test_correct_report() {
    let house = initialize_house();
    let socket = Socket::new("Socket".to_owned(), Power::Watt(1350.0));
    let provider = OwningDeviceInfoProvider::new(socket);
    let expected = "
       House: [Paradise]

        Room: [Living room]
      
      Device: Socket
       Power: 1350.00W
       State: On

      Device: Thermo not found
";
    assert_eq!(house.create_report(&provider), expected);
}

#[test]
fn test_incorrect_report() {
    let house = initialize_house();
    let socket = Socket::new("Socket".to_owned(), Power::Watt(1350.0));
    let provider = OwningDeviceInfoProvider::new(socket);
    let expected = "
       House: [Paradise]


Room: [Living room]
      Device: Socket1

       Power: 1250.00W

       State: On

Error! Device Thermo not found.
";
    assert_ne!(house.create_report(&provider), expected);
}
