use anyhow::Result;
use std::thread;
use std::time::Duration;
use tokio::{self, net::UdpSocket};

use udp_smart_thermo::BIND_IP;

#[tokio::main]
async fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let address = args.next().unwrap_or_else(|| BIND_IP.to_owned());

    println!("Receiver address from args: {address}");
    let socket = UdpSocket::bind(&address)
        .await
        .expect("couldn't bind to address");

    let mut buf = [0; 4];
    loop {
        let change_temperature: f32 = 0.05;
        let bytes = change_temperature.to_be_bytes();
        let (_, addr) = socket.recv_from(&mut buf).await?;
        let result = socket.send_to(&bytes, addr).await;
        if let Err(err) = result {
            println!("couldn't send temperature: {err}")
        }
        thread::sleep(Duration::from_secs(1))
    }
}
