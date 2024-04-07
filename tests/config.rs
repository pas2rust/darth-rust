use chrono::{DateTime, Local};
use darth_rust::DarthRust;
use serde::{Deserialize, Serialize};

#[derive(DarthRust, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub password: String,
    pub email: String,
    pub age: u8,
    pub friends: Vec<User>,
}

#[derive(Debug, DarthRust, Clone)]
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

#[derive(DarthRust, Debug)]
pub struct Test<T, A> {
    pub item: T,
    pub a: A,
    pub date: DateTime<Local>
}
