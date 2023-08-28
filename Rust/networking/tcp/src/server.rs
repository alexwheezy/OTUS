use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

fn process_stream(mut stream: TcpStream) {
    let mut buf = [0u8; 4];
    loop {
        if stream.read_exact(&mut buf).is_err() {
            break;
        }
        let request = u32::from_be_bytes(buf);
        println!("Request: {request}");

        let reply = request ^ 0xFFFF_FFFF;
        if stream.write_all(&reply.to_be_bytes()).is_err() {
            break;
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9090").expect("bind failed");

    while let Some(stream) = listener.incoming().next() {
        if stream.is_err() {
            continue;
        }

        let stream = stream.unwrap();
        let peer = stream.peer_addr();

        println!("Connected: {peer:?}");
        thread::spawn(move || process_stream(stream));
        println!("Disconnected: {peer:?}");
    }
}
