#![allow(dead_code)]
#![allow(unused_variables)]

use smart_house::{
    devices::smart::socket::Socket,
    units::{physics::Power, Enable},
};

fn main() {
    //This example demonstrates how we can query and control digital devices in the rooms.
    let mut socket = Socket::new("Socket".to_owned(), Power::Watt(1350.0));
    println!("{}", socket.description());

    //Setting up new settings and options.
    socket.set_power(Power::Kilowatt(1.35));
    socket.switch(Enable::Off);

    println!("{}", socket.description());
}
