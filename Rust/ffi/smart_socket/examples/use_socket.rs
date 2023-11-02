use std::{
    ffi::{c_char, CStr, CString},
    fmt::Display,
};

#[link(name = "smart_socket", kind = "dylib")]
extern "C" {
    fn smart_socket_create(name: *const c_char) -> *mut SmartSocket;
    fn smart_socket_get_power(ptr: *const SmartSocket) -> Power;
    fn smart_socket_is_enabled(ptr: *const SmartSocket) -> bool;
    fn smart_socket_switch(ptr: *mut SmartSocket);
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub enum Enabled {
    On,
    Off,
}

impl Display for Enabled {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Enabled::On => write!(f, "On"),
            Enabled::Off => write!(f, "Off"),
        }
    }
}

pub type Power = f32;

#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub struct SmartSocket {
    name: *const c_char,
    power: Power,
    state: Enabled,
}

impl Default for SmartSocket {
    fn default() -> Self {
        Self::new("")
    }
}

impl SmartSocket {
    pub fn new(name: &str) -> Self {
        unsafe { *smart_socket_create(CString::new(name).unwrap().into_raw()) }
    }

    pub fn description(&self) -> String {
        format!(
            r#"
       Name:  {name}
       Power: {power:.2} kWh
       State: {state}
            "#,
            name = unsafe { CStr::from_ptr(self.name).to_str().unwrap() },
            power = self.power,
            state = self.state
        )
    }

    pub fn power(&self) -> Power {
        unsafe { smart_socket_get_power(self) }
    }

    pub fn is_enabled(&self) -> bool {
        unsafe { smart_socket_is_enabled(self) }
    }

    pub fn switch(&mut self) {
        unsafe { smart_socket_switch(self) }
    }
}

fn main() {
    let mut smart_socket = SmartSocket::new("Socket");
    println!("{}", smart_socket.description());

    assert!(smart_socket.is_enabled());
    assert_eq!(smart_socket.power(), 3.45);

    smart_socket.switch();

    assert!(!smart_socket.is_enabled());
    assert_eq!(smart_socket.power(), 0.0);

    println!("{}", smart_socket.description());
}
