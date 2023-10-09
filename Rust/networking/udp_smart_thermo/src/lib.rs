use anyhow::{Context, Result};
use std::fmt::Display;
use std::sync::{Arc, Mutex};
use tokio::net::{ToSocketAddrs, UdpSocket};

type Temperature = f32;

pub const BIND_IP: &str = "0.0.0.0:8080";

#[derive(Debug)]
pub struct SmartThermo {
    temperature: Arc<Mutex<Temperature>>,
}

// Beautiful temperature printout
impl Display for SmartThermo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.2}°C", *self.temperature.lock().unwrap())
    }
}

impl SmartThermo {
    pub async fn new(address: impl ToSocketAddrs, temperature: Temperature) -> Result<SmartThermo> {
        let socket = UdpSocket::bind(&address)
            .await
            .context("couldn't bind to address")?;

        let temperature = Arc::new(Mutex::new(temperature));
        let temperature_clone = Arc::clone(&temperature);
        tokio::spawn(async move {
            loop {
                let mut buf = [0; 4];
                socket.send_to(&buf, BIND_IP).await.unwrap();
                if let Err(err) = socket.recv_from(&mut buf).await {
                    println!("can't receive datagram: {err}");
                }

                let value = f32::from_be_bytes(buf);
                *temperature_clone.lock().unwrap() += value;
            }
        });

        Ok(Self { temperature })
    }

    // Current temperature value.
    pub fn get(&self) -> Temperature {
        *self.temperature.lock().unwrap()
    }

    // Update temperature value.
    pub fn set(&mut self, value: Temperature) {
        *self.temperature.lock().unwrap() = value;
    }
}

#[test]
fn test_get_temperature() {
    let thermo = SmartThermo {
        temperature: Arc::new(Mutex::new(35.5)),
    };
    assert_eq!(*thermo.temperature.lock().unwrap(), 35.5);
}

#[test]
fn test_new_set_temperature() {
    let mut thermo = SmartThermo {
        temperature: Arc::new(Mutex::new(35.5)),
    };
    thermo.set(30.5);
    assert_eq!(*thermo.temperature.lock().unwrap(), 30.5);
}
