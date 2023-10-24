use std::sync::Arc;

use anyhow::{Context, Result};
use gui_smart_socket::{SmartSocket, CMD_NUMS, LOCAL_IP};

use tokio::{
    self,
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    sync::Mutex,
};

#[tokio::main]
async fn main() -> Result<()> {
    let mut args = std::env::args();
    args.next().unwrap();

    let address = args.next().unwrap_or_else(|| LOCAL_IP.into());
    println!("Server running...");

    let listener = TcpListener::bind(address)
        .await
        .context("Can't bind tcp listener")?;

    let smart_socket = Arc::new(Mutex::new(SmartSocket::default()));

    loop {
        let smart_socket_clone = Arc::clone(&smart_socket);
        let (mut stream, _) = listener
            .accept()
            .await
            .context("Can't receive connection")?;

        tokio::spawn(async move {
            let peer = stream
                .peer_addr()
                .map(|a| a.to_string())
                .unwrap_or_else(|_| "Unknown".into());
            println!("Peer '{peer}' connected");

            let mut in_buffer = [0u8];
            while stream.read_exact(&mut in_buffer).await.is_ok() {
                let response = smart_socket_clone
                    .lock()
                    .await
                    .process_command(in_buffer[0].into());
                let response_buf: [u8; CMD_NUMS] = response.into();
                if stream.write_all(&response_buf).await.is_err() {
                    break;
                };
            }

            println!("Connection with {peer} lost. Waiting for new connections...");
        });
    }
}
