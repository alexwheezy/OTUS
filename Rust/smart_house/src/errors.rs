#[derive(Debug, PartialEq)]
pub enum Error {
    Device(DeviceError),
}

#[derive(Debug, PartialEq)]
pub enum DeviceError {
    NotFound(String),
    NotConnetion(String),
    InvalidData(String),
    UnknownError,
}
