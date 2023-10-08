// A module for working with a smart socket and its simulator.

use std::{
    fmt,
    io::{Read, Write},
    net::{TcpStream, ToSocketAddrs},
};

use anyhow::{Context, Result};

#[derive(Debug)]
pub struct SmartSocketClient {
    stream: TcpStream,
}

const CMD_NUMS: usize = 5;

impl SmartSocketClient {
    pub fn new(addr: impl ToSocketAddrs) -> Result<Self> {
        let stream = TcpStream::connect(&addr).with_context(|| {
            let ip = addr.to_socket_addrs().unwrap().next().unwrap().ip();
            format!("Unable to connect to server: {}", ip)
        })?;
        Ok(Self { stream })
    }

    pub fn run_command(&mut self, command: Command) -> Result<Response> {
        self.stream.write_all(&[command.into()])?;
        let mut buffer = [0u8; CMD_NUMS];
        self.stream.read_exact(&mut buffer)?;
        Ok(buffer.into())
    }
}

pub enum Command {
    TurnOff,
    TurnOn,
    IsEnabled,
    GetPower,
    Unknown,
}

impl From<u8> for Command {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::TurnOff,
            1 => Self::TurnOn,
            2 => Self::IsEnabled,
            3 => Self::GetPower,
            _ => Self::Unknown,
        }
    }
}

impl From<Command> for u8 {
    fn from(cmd: Command) -> Self {
        match cmd {
            Command::TurnOff => 0,
            Command::TurnOn => 1,
            Command::IsEnabled => 2,
            Command::GetPower => 3,
            Command::Unknown => 255,
        }
    }
}

pub enum Response {
    Ok,
    Enabled,
    Disabled,
    Power(f32),
    Unknown,
}

impl From<[u8; CMD_NUMS]> for Response {
    fn from(bytes: [u8; CMD_NUMS]) -> Self {
        match bytes {
            [0, ..] => Self::Ok,
            [1, ..] => Self::Enabled,
            [2, ..] => Self::Disabled,
            [3, ..] => {
                let mut buf = [0u8; 4];
                buf.copy_from_slice(&bytes[1..]);
                Self::Power(f32::from_be_bytes(buf))
            }
            _ => Self::Unknown,
        }
    }
}

impl From<Response> for [u8; CMD_NUMS] {
    fn from(resp: Response) -> Self {
        let mut buffer = [0u8; CMD_NUMS];
        match resp {
            Response::Ok => {}
            Response::Enabled => buffer[0] = 1,
            Response::Disabled => buffer[0] = 2,
            Response::Power(pwr) => {
                buffer[0] = 3;
                buffer[1..].copy_from_slice(&pwr.to_be_bytes())
            }
            Response::Unknown => buffer[0] = 255,
        };
        buffer
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Response::Ok => write!(f, "Ok"),
            Response::Enabled => write!(f, "Enabled"),
            Response::Disabled => write!(f, "Disabled"),
            Response::Power(power) => write!(f, "Power: {}", power),
            Response::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct SmartSocket {
    enabled: bool,
}

impl SmartSocket {
    pub fn process_command(&mut self, cmd: Command) -> Response {
        match cmd {
            Command::TurnOn => {
                self.enabled = true;
                Response::Ok
            }
            Command::TurnOff => {
                self.enabled = false;
                Response::Ok
            }
            Command::IsEnabled => {
                if self.enabled {
                    Response::Enabled
                } else {
                    Response::Disabled
                }
            }
            Command::GetPower => {
                if self.enabled {
                    Response::Power(220.5)
                } else {
                    Response::Power(0.0)
                }
            }
            Command::Unknown => {
                eprintln!("Unknown command received");
                Response::Unknown
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_command() {
        let mut smart_socket = SmartSocket { enabled: true };
        smart_socket.process_command(Command::IsEnabled);
        assert!(smart_socket.enabled);

        if let Response::Power(value) = smart_socket.process_command(Command::GetPower) {
            assert_eq!(value, 220.5);
        }
    }

    #[test]
    fn test_unknow_command() {
        let mut smart_socket = SmartSocket { enabled: true };
        assert_eq!(
            smart_socket.process_command(Command::Unknown).to_string(),
            "Unknown".to_owned()
        );
    }

    #[test]
    fn test_device_enabled() {
        let mut smart_socket = SmartSocket { enabled: true };
        smart_socket.process_command(Command::IsEnabled);
        smart_socket.process_command(Command::TurnOn);
        assert!(smart_socket.enabled);

        smart_socket.process_command(Command::TurnOff);
        assert!(!smart_socket.enabled);
    }
}
