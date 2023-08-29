use anyhow::{Context, Result};
use std::{
    io::{Read, Write},
    net::TcpListener,
};

use tcp_smart_socket::SmartSocket;

const LOCAL_IP: &str = "127.0.0.1:9090";
const CMD_NUMS: usize = 5;

fn main() -> Result<()> {
    let mut args = std::env::args();
    args.next().unwrap();

    let address = args.next().unwrap_or_else(|| LOCAL_IP.into());
    println!("Server running...");

    let listener = TcpListener::bind(address).context("Can't bind tcp listener")?;
    let mut smart_socket = SmartSocket::default();

    while let Some(connection) = listener.incoming().next() {
        let mut stream = match connection {
            Ok(conn) => conn,
            Err(err) => {
                println!("Can't receive connection: {err}");
                continue;
            }
        };

        let peer = stream
            .peer_addr()
            .map(|a| a.to_string())
            .unwrap_or_else(|_| "Unknown".into());
        println!("Peer '{peer}' connected");

        let mut in_buffer = [0u8];
        while stream.read_exact(&mut in_buffer).is_ok() {
            let response = smart_socket.process_command(in_buffer[0].into());
            let response_buf: [u8; CMD_NUMS] = response.into();
            if stream.write_all(&response_buf).is_err() {
                break;
            };
        }

        println!("Connection with {peer} lost. Waiting for new connections...");
    }
    Ok(())
}
