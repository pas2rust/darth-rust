use darth_rust::DarthRust;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub enum Enum {
    #[default]
    Identity,
    Reverse,
}

impl Enum {
    pub fn call(&self, s: &str) -> String {
        match self {
            Enum::Identity => s.to_string(),
            Enum::Reverse => s.chars().rev().collect(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, DarthRust)]
pub struct E {
    test: Enum,
}

#[test]
fn _enum() {
    let e = E::new().test(Enum::Identity);
    let test = e.test;
    assert!(test.call("").is_empty())
}
