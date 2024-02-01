use darth_rust::DarthRust;
use serde::{Deserialize, Serialize};
#[derive(Debug, DarthRust)]
pub struct CalcStruct {
    pub usize: usize,
    pub u8: u8,
    pub u16: u16,
    pub u32: u32,
    pub u64: u64,
    pub u128: u128,
    pub isize: isize,
    pub i16: i16,
    pub i32: i32,
    pub i64: i64,
    pub i128: i128,
    pub f64: f64,
    pub f32: f32,
}

#[derive(DarthRust, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct User {
    id: String,
    name: String,
    password: String,
    email: String,
    age: u8,
    friends: Vec<User>,
}

impl User {
    pub fn default() -> Self {
        Self {
            age: 0,
            email: "".to_string(),
            friends: vec![],
            id: "".to_string(),
            name: "".to_string(),
            password: "".to_string(),
        }
    }
}
