use crate::devices::smart::{socket::Socket, thermo::Thermometer};

pub trait DeviceInfoProvider {
    fn status(&self, device: &str) -> String;
}

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
