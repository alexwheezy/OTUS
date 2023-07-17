use crate::devices::smart::{socket::Socket, thermo::Thermometer};
use crate::errors::DeviceError;

pub trait DeviceInfoProvider {
    fn status(&self, device: &str) -> Result<String, DeviceError>;
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
    fn status(&self, device: &str) -> Result<String, DeviceError> {
        match self.socket.name() == device {
            true => Ok(self.socket.description()),
            false => Err(DeviceError::NotFound(device.to_owned())),
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
        Self { socket, thermo }
    }
}

impl<'a, 'b> DeviceInfoProvider for BorrowingDeviceInfoProvider<'a, 'b> {
    fn status(&self, device: &str) -> Result<String, DeviceError> {
        if self.socket.name() == device {
            Ok(self.socket.description())
        } else if self.thermo.name() == device {
            Ok(self.thermo.description())
        } else {
            Err(DeviceError::NotFound(device.to_owned()))
        }
    }
}
