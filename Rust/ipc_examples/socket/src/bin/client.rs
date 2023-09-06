use std::{io::stdin, net::UdpSocket};

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:9090").expect("can't bind socket");
    loop {
        let mut str = String::new();
        stdin().read_line(&mut str).unwrap();
        socket
            .send_to(str.as_bytes(), "127.0.0.1:9091")
            .expect("can't send datagram");
    }
}
