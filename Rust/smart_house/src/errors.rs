use std::{
    error::Error,
    fmt::{Debug, Display},
};

#[derive(Debug, PartialEq)]
pub struct CheckStatusError {
    pub(crate) kind: DeviceErrorKind,
}

#[derive(Debug, PartialEq)]
pub enum DeviceErrorKind {
    NotFound(String),
    NotConnetion(String),
    InvalidData(String),
    UnknownError,
}

impl CheckStatusError {
    pub fn kind(&self) -> &DeviceErrorKind {
        &self.kind
    }
}

impl Display for CheckStatusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl Display for DeviceErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeviceErrorKind::NotFound(device) => write!(f, "{device} not found"),
            DeviceErrorKind::NotConnetion(device) => {
                write!(f, "{device} not connection")
            }
            DeviceErrorKind::InvalidData(device) => {
                write!(f, "{device} is transmitting incorrect data")
            }
            DeviceErrorKind::UnknownError => write!(f, "unknown error"),
        }
    }
}

impl Error for DeviceErrorKind {}

impl Error for CheckStatusError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.kind)
    }
}
