use darth_rust::DarthRust;
use serde::{Deserialize, Serialize};
use serde_json::json;
#[derive(DarthRust, Debug, Serialize, Deserialize, PartialEq, Clone)]
struct User {
    id: String,
    name: String,
    password: String,
    email: String,
    age: u8,
    friends: Vec<User>,
}

impl User {
    fn default() -> Self {
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

#[test]
pub fn formatters() {
    let friends = vec![User::default(); 10];
    let user_new = User::new(
        "id".into(),
        "name".into(),
        "password".into(),
        "email".into(),
        18,
        friends.clone(),
    );

    let user_json_value = user_new.to_json();
    let user_expected_json = json!({
        "id": *user_new.get_id(),
        "name": *user_new.get_name(),
        "password": *user_new.get_password(),
        "email": *user_new.get_email(),
        "age": *user_new.get_age(),
        "friends": *user_new.get_friends(),
    });
    let user_from_json = User::from_json(user_json_value.clone()).unwrap();
    assert_eq!(user_new, user_from_json);
    assert_eq!(user_json_value, user_expected_json);
}
