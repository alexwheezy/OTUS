use std::{
    io::{Read, Write},
    net::TcpStream,
    thread,
    time::Duration,
};

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:9090").expect("Connection failed");

    let mut buf = [0u8; 4];
    for i in 0..10u32 {
        stream.write_all(&i.to_be_bytes()).expect("Fail to request");
        stream.read_exact(&mut buf).expect("Fail to get reply");
        println!("Reply: {}", u32::from_be_bytes(buf));
        thread::sleep(Duration::from_secs(1));
    }
}
