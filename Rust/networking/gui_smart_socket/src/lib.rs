// A module for working with a smart socket and its simulator.

use anyhow::{Context, Result};
use eframe::egui;
use rand::Rng;
use std::thread;
use std::{
    fmt,
    io::{Read, Write},
    net::{TcpStream, ToSocketAddrs},
    sync::{Arc, Mutex},
};

#[derive(Debug)]
pub struct SmartSocketClient {
    stream: TcpStream,
    connected: bool,
}

pub const LOCAL_IP: &str = "127.0.0.1:9090";
pub const CMD_NUMS: usize = 5;

impl SmartSocketClient {
    pub fn new(addr: impl ToSocketAddrs) -> Result<Self> {
        let stream = TcpStream::connect(&addr).with_context(|| {
            let ip = addr.to_socket_addrs().unwrap().next().unwrap().ip();
            format!("Unable to connect to server: {}", ip)
        })?;

        Ok(Self {
            stream,
            connected: false,
        })
    }

    pub fn is_connected(&self) -> bool {
        self.connected
    }

    pub fn run_command(&mut self, command: Command) -> Result<Response> {
        self.stream.write_all(&[command.into()])?;
        let mut buffer = [0u8; CMD_NUMS];
        self.stream.read_exact(&mut buffer)?;
        Ok(buffer.into())
    }
}

fn setup_custom_frame() -> egui::Frame {
    egui::containers::Frame {
        inner_margin: egui::style::Margin {
            left: 2.,
            right: 2.,
            top: 2.,
            bottom: 2.,
        },
        outer_margin: egui::style::Margin {
            left: 2.,
            right: 2.,
            top: 2.,
            bottom: 2.,
        },
        rounding: egui::Rounding {
            nw: 9.0,
            ne: 9.0,
            sw: 9.0,
            se: 9.0,
        },
        shadow: eframe::epaint::Shadow {
            extrusion: 0.0,
            color: egui::Color32::BLACK,
        },
        fill: egui::Color32::from_rgb(7, 110, 213),
        stroke: egui::Stroke::new(3.0, egui::Color32::BLACK),
    }
}

impl eframe::App for SmartSocketClient {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(setup_custom_frame())
            .show(ctx, |ui| {
                ctx.request_repaint_after(std::time::Duration::from_millis(500));

                // HDPI Window Scaling
                ctx.set_pixels_per_point(1.5);
                ui.vertical_centered(|ui| {
                    ui.colored_label(egui::Color32::from_rgb(255, 255, 255), "Smart Plug");
                    ui.add_space(30.0);
                    ui.add(
                        egui::Image::new(egui::include_image!("../images/smart-socket_256.png"))
                            .max_width(200.0),
                    );

                    ui.add_space(30.0);
                    let enabled_label = if !self.is_connected() {
                        "Power Off"
                    } else {
                        "Power On"
                    };

                    ui.colored_label(egui::Color32::from_rgb(255, 255, 255), enabled_label);
                    ui.add_space(10.0);

                    ui.colored_label(
                        egui::Color32::from_rgb(255, 255, 255),
                        format!("⚡{} kWh", self.run_command(Command::GetPower).unwrap()),
                    );

                    ui.colored_label(
                        egui::Color32::from_rgb(255, 255, 255),
                        format!("⚡{} A", self.run_command(Command::GetAmp).unwrap()),
                    );

                    ui.add_space(10.0);
                });
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    ui.add_space(25.0);
                    let power_button = egui::ImageButton::new(egui::include_image!(
                        "../images/power-button_128.png"
                    ))
                    .frame(false);
                    if ui.add(power_button).clicked() {
                        if self.is_connected() {
                            self.connected = false;
                            self.run_command(Command::TurnOff).unwrap();
                        } else {
                            self.connected = true;
                            self.run_command(Command::TurnOn).unwrap();
                        }
                    }
                });
            });
    }
}

pub enum Command {
    TurnOff,
    TurnOn,
    IsEnabled,
    GetPower,
    GetAmp,
    Unknown,
}

impl From<u8> for Command {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::TurnOff,
            1 => Self::TurnOn,
            2 => Self::IsEnabled,
            3 => Self::GetPower,
            4 => Self::GetAmp,
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
            Command::GetAmp => 4,
            Command::Unknown => 255,
        }
    }
}

pub enum Response {
    Ok,
    Enabled,
    Disabled,
    Power(f32),
    Amp(f32),
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
            [4, ..] => {
                let mut buf = [0u8; 4];
                buf.copy_from_slice(&bytes[1..]);
                Self::Amp(f32::from_be_bytes(buf))
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
            Response::Amp(amp) => {
                buffer[0] = 4;
                buffer[1..].copy_from_slice(&amp.to_be_bytes())
            }
            Response::Unknown => buffer[0] = 255,
        };
        buffer
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Response::Power(power) => write!(f, "{:.2}", power),
            Response::Amp(amp) => write!(f, "{:.1}", amp),
            _ => write!(f, "Unknown"),
        }
    }
}

#[derive(Debug)]
pub struct SmartSocket {
    enabled: bool,
    power: Arc<Mutex<f32>>,
    amp: f32,
}

impl Default for SmartSocket {
    fn default() -> Self {
        let power = Arc::new(Mutex::new(0.0));
        let power_clone = Arc::clone(&power);

        thread::spawn(move || {
            loop {
                // updating power indicators
                *power_clone.lock().unwrap() = rand::thread_rng().gen_range(3.45..3.52);
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
        });
        Self {
            enabled: false,
            power,
            amp: 0.0,
        }
    }
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
                    Response::Power(*self.power.lock().unwrap())
                } else {
                    Response::Power(0.0)
                }
            }
            Command::GetAmp => {
                if self.enabled {
                    Response::Amp(16.0)
                } else {
                    Response::Amp(self.amp)
                }
            }
            Command::Unknown => {
                eprintln!("Unknown command received");
                Response::Unknown
            }
        }
    }
}
