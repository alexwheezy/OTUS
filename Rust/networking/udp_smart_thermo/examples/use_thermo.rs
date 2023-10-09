use anyhow::Result;
use std::{thread, time::Duration};
use tokio;
use udp_smart_thermo::SmartThermo;

#[tokio::main]
async fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let address = args.next().unwrap_or_else(|| "127.0.0.8081".to_owned());
    let thermo = SmartThermo::new(address, 35.5).await?;
    loop {
        thread::sleep(Duration::from_secs(1));
        println!("Temp: {thermo}");
    }
}
