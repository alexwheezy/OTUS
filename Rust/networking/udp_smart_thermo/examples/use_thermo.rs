use std::{thread, time::Duration};
use udp_smart_thermo::{SmartThermo, LISTEN_IP};

fn main() {
    let address = LISTEN_IP;
    let thermo = SmartThermo::new(address, 35.5).unwrap();
    loop {
        thread::sleep(Duration::from_secs(1));
        println!("Temp: {thermo}");
    }
}
