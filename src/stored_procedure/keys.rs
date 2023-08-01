// Struct to hold all the different types of keys and identifiers
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]

pub struct Keys {
    m: Option<String>,
    y: Option<String>,
    e: Option<String>,
    p: Option<String>,
    pk: Option<String>,
    s: Option<String>,
    z1: Option<String>,
    z2: Option<String>,
    z3: Option<String>,
    z4: Option<String>,
    z5: Option<String>,
}

#[allow(dead_code)] //remove for testing purposes
impl Keys {
    // Create a new Keys object with all fields None
    pub fn new() -> Self {
        Keys {
            ..Default::default()
        }
    }

    // Setter functions
    pub fn set_m(&mut self, value: String) {
        self.m = Some(value);
    }

    pub fn set_y(&mut self, value: String) {
        self.y = Some(value);
    }

    pub fn set_e(&mut self, value: String) {
        self.e = Some(value);
    }

    pub fn set_p(&mut self, value: String) {
        self.p = Some(value);
    }

    pub fn set_pk(&mut self, value: String) {
        self.pk = Some(value);
    }

    pub fn set_s(&mut self, value: String) {
        self.s = Some(value);
    }

    pub fn set_z1(&mut self, value: String) {
        self.z1 = Some(value);
    }

    pub fn set_z2(&mut self, value: String) {
        self.z2 = Some(value);
    }

    pub fn set_z3(&mut self, value: String) {
        self.z3 = Some(value);
    }

    pub fn set_z4(&mut self, value: String) {
        self.z4 = Some(value);
    }

    pub fn set_z5(&mut self, value: String) {
        self.z5 = Some(value);
    }

    // Getter functions
    pub fn get_m(&self) -> Option<&String> {
        self.m.as_ref()
    }

    pub fn get_y(&self) -> Option<&String> {
        self.y.as_ref()
    }

    pub fn get_e(&self) -> Option<&String> {
        self.e.as_ref()
    }

    pub fn get_p(&self) -> Option<&String> {
        self.p.as_ref()
    }

    pub fn get_pk(&self) -> Option<&String> {
        self.pk.as_ref()
    }

    pub fn get_s(&self) -> Option<&String> {
        self.s.as_ref()
    }

    pub fn get_z1(&self) -> Option<&String> {
        self.z1.as_ref()
    }

    pub fn get_z2(&self) -> Option<&String> {
        self.z2.as_ref()
    }

    pub fn get_z3(&self) -> Option<&String> {
        self.z3.as_ref()
    }

    pub fn get_z4(&self) -> Option<&String> {
        self.z4.as_ref()
    }

    pub fn get_z5(&self) -> Option<&String> {
        self.z5.as_ref()
    }
}
