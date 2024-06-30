#![cfg(feature = "json")]
use darth_rust::DarthRust;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(
    DarthRust,
    Deserialize,
    Serialize,
    Debug,
    PartialEq,
    Clone,
)]
pub struct User {
    pub id: String,
    pub name: String,
    pub password: String,
    pub email: String,
    pub age: u8,
    pub friends: Vec<User>,
}

#[test]
fn json() {
    let friends = vec![User::default(); 10];
    let user_new = User::new()
        .id("id")
        .name("name")
        .password("password")
        .email("email")
        .age(18)
        .friends(friends.clone());
    let user_json_value = user_new.to_json();
    let user_expected_json = json!({
        "id": user_new.id,
        "name": user_new.name,
        "password": user_new.password,
        "email": user_new.email,
        "age": user_new.age,
        "friends": user_new.friends,
    });
    let user_from_json =
        User::from_json(user_json_value.clone()).unwrap();
    assert_eq!(user_new, user_from_json);
    assert_eq!(user_json_value, user_expected_json);
}
