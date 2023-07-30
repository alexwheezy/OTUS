#![allow(unused_assignments)]
use crate::devices::smart::{socket::Socket, thermo::Thermometer};
use crate::errors::{CheckStatusError, DeviceErrorKind};

pub trait DeviceInfoProvider {
    fn status(&self, device: &str) -> Result<String, CheckStatusError>;
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
    fn status(&self, device: &str) -> Result<String, CheckStatusError> {
        let socket_name = self.socket.name();
        match socket_name == device {
            true => Ok(self.socket.description()),
            false => Err(CheckStatusError {
                kind: DeviceErrorKind::NotFound(socket_name.to_owned()),
            }),
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
    fn status(&self, device: &str) -> Result<String, CheckStatusError> {
        let mut device_name = self.socket.name();
        if self.socket.name() == device {
            return Ok(self.socket.description());
        } else if self.thermo.name() == device {
            device_name = self.thermo.name();
            return Ok(self.thermo.description());
        }
        Err(CheckStatusError {
            kind: DeviceErrorKind::NotFound(device_name.to_owned()),
        })
    }
}
