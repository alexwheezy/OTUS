use std::net::UdpSocket;
use std::thread;
use std::time::Duration;

use udp_smart_thermo::{BIND_IP, LISTEN_IP};

fn main() {
    let mut args = std::env::args().skip(1);
    let address = args.next().unwrap_or_else(|| BIND_IP.to_owned());

    println!("Receiver address from args: {address}");
    let socket = UdpSocket::bind(address).expect("couldn't bind to address");

    println!("Starting send temperature from {BIND_IP} to {LISTEN_IP}");

    loop {
        let change_temperature: f32 = 0.05;
        let bytes = change_temperature.to_be_bytes();
        let result = socket.send_to(&bytes, LISTEN_IP);
        if let Err(err) = result {
            println!("couldn't send temperature: {err}")
        }
        thread::sleep(Duration::from_secs(1))
    }
}
