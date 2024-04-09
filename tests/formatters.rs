#![cfg(all(feature = "build", feature = "json"))]
mod config;
pub use config::User;
use serde_json::json;

#[test]
pub fn formatters() {
    let friends = vec![User::default(); 10];
    let user_new = User::new()
        .id("id")
        .name("name")
        .password("password")
        .email("email")
        .age(18)
        .friends(friends.clone())
        .build()
        .unwrap();
    let user_json_value = user_new.to_json();
    let user_expected_json = json!({
        "id": *user_new.get_id(),
        "name": *user_new.get_name(),
        "password": *user_new.get_password(),
        "email": *user_new.get_email(),
        "age": *user_new.get_age(),
        "friends": *user_new.get_friends(),
    });
    let user_from_json =
        User::from_json(user_json_value.clone()).unwrap();
    assert_eq!(user_new, user_from_json);
    assert_eq!(user_json_value, user_expected_json);
}
