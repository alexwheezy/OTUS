use crate::devices::smart::{socket::Socket, thermo::Thermometer};

pub trait DeviceInfoProvider {
    fn status(&self, device: &str) -> String;
}

#[derive(Debug, PartialEq)]
pub struct OwningDeviceInfoProvider {
    socket: Socket,
}

impl OwningDeviceInfoProvider {
    pub fn new(socket: Socket) -> Self {
        Self { socket }
    }
}

impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn status(&self, device: &str) -> String {
        match self.socket.name() == device {
            true => self.socket.description(),
            false => format!("\nError! Device {} not found.\n", device),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct BorrowingDeviceInfoProvider<'a, 'b> {
    socket: &'a Socket,
    thermo: &'b Thermometer,
}

impl<'a, 'b> BorrowingDeviceInfoProvider<'a, 'b> {
    pub fn new(socket: &'a Socket, thermo: &'b Thermometer) -> Self {
        Self {
            socket: &socket,
            thermo: &thermo,
        }
    }
}

impl<'a, 'b> DeviceInfoProvider for BorrowingDeviceInfoProvider<'a, 'b> {
    fn status(&self, device: &str) -> String {
        if self.socket.name() == device {
            self.socket.description()
        } else if self.thermo.name() == device {
            self.thermo.description()
        } else {
            format!("\nError! Device {} not found.\n", device)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::physics::Power;
    use crate::units::physics::Temperature;

    fn devices() -> (Socket, Thermometer) {
        (
            Socket::new("Socket".to_owned(), Power::Watt(0.35)),
            Thermometer::new("Thermo".to_owned(), Temperature::Celsius(30.1)),
        )
    }

    #[test]
    fn test_constructed() {
        let (socket, thermo) = devices();
        let _borrowing_provider = BorrowingDeviceInfoProvider::new(&socket, &thermo);
        let _owning_provider = OwningDeviceInfoProvider::new(socket);
    }

    #[test]
    fn test_borrowing_status() {
        let (socket, thermo) = devices();
        let borrowing_provider = BorrowingDeviceInfoProvider::new(&socket, &thermo);
        assert_eq!(borrowing_provider.status("Socket"), socket.description());
        assert_eq!(borrowing_provider.status("Thermo"), thermo.description());
    }

    #[test]
    fn test_owning_status() {
        let (socket, _) = devices();
        let owning_provider = OwningDeviceInfoProvider::new(socket);
        assert_eq!(
            owning_provider.status("Socket"),
            Socket::new("Socket".to_owned(), Power::Watt(0.35)).description()
        );
    }
}
