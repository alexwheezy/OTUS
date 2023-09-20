use thiserror::Error;

#[derive(Error, Debug)]
pub enum DeviceError {
    #[error("{0} device not found")]
    NotFound(String),
    #[error("{0} not connection")]
    NotConnetion(String),
    #[error("{0} is transmitting incorrect data")]
    InvalidData(String),
    #[error("unknown error")]
    UnknownError,
}
