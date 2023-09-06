use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:9091").expect("could bind to address");
    loop {
        let mut buf = [0; 1024];
        let (number_of_bytes, _) = socket.recv_from(&mut buf).expect("Didn't receive data");
        let str = String::from_utf8_lossy(&buf[..number_of_bytes]);
        print!("{}", str);
    }
}
