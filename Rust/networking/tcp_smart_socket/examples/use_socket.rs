use anyhow::Result;
use std::io;
use tcp_smart_socket::{Command, SmartSocketClient};

const LOCAL_IP: &str = "127.0.0.1:9090";

fn main() -> Result<()> {
    let mut client = SmartSocketClient::new(LOCAL_IP)?;

    loop {
        show_menu();
        let input = read_input();
        let response = match input {
            Some(command) => client.run_command(command).unwrap(),
            None => {
                break;
            }
        };
        println!("Response: {response}");
    }
    Ok(())
}

fn show_menu() {
    println!();
    println!("Help action:");
    println!("1) Turn off");
    println!("2) Turn on");
    println!("3) Is enabled");
    println!("4) Power");
    println!("q) Exit");
}

fn read_input() -> Option<Command> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let cmd = match input.trim() {
        "1" => Command::TurnOff,
        "2" => Command::TurnOn,
        "3" => Command::IsEnabled,
        "4" => Command::GetPower,
        "q" => return None,
        _ => Command::Unknown,
    };

    Some(cmd)
}
