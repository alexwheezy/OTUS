#![allow(clippy::missing_safety_doc)]

use std::ffi::c_char;

#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub enum Enabled {
    On,
    Off,
}

pub type Power = f32;

#[repr(C)]
#[derive(Clone, PartialEq)]
pub struct SmartSocket {
    name: *const c_char,
    power: Power,
    state: Enabled,
}

impl Default for SmartSocket {
    fn default() -> Self {
        Self {
            name: "Socket".as_ptr() as *const c_char,
            power: 3.45,
            state: Enabled::On,
        }
    }
}

impl SmartSocket {
    pub fn new(name: *const c_char) -> Self {
        Self {
            name,
            ..Default::default()
        }
    }

    pub fn power(&self) -> Power {
        self.power
    }

    pub fn is_enabled(&self) -> bool {
        match self.state {
            Enabled::On => true,
            Enabled::Off => false,
        }
    }

    pub fn switch(&mut self) {
        match self.state {
            Enabled::On => {
                self.state = Enabled::Off;
                self.power = 0.0;
            }
            Enabled::Off => {
                self.state = Enabled::On;
                self.power = 3.45;
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn smart_socket_create(name: *const c_char) -> *mut SmartSocket {
    Box::into_raw(Box::new(SmartSocket::new(name)))
}

#[no_mangle]
pub unsafe extern "C" fn smart_socket_get_power(ptr: *const SmartSocket) -> Power {
    assert!(!ptr.is_null());
    (*ptr).power()
}

#[no_mangle]
pub unsafe extern "C" fn smart_socket_is_enabled(ptr: *const SmartSocket) -> bool {
    assert!(!ptr.is_null());
    (*ptr).is_enabled()
}

#[no_mangle]
pub unsafe extern "C" fn smart_socket_switch(ptr: *mut SmartSocket) {
    assert!(!ptr.is_null());
    (*ptr).switch()
}
