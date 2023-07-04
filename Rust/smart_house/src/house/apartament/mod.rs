#![allow(unused_variables)]
#![allow(dead_code)]

#[derive(Debug, Clone)]
pub struct Apartament {
    name: String,
    devices: Vec<String>,
}

impl Apartament {
    pub fn new(name: &str, devices: Vec<String>) -> Self {
        Self {
            name: name.to_owned(),
            devices,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn devices(&self) -> &Vec<String> {
        &self.devices
    }
}
