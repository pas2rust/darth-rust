use darth_rust::DarthRust;
use serde::{Deserialize, Serialize};
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
pub fn setters_and_getters() {
    let mut user_default = User::default();
    let friends = vec![User::default(); 10];
    let mut user_new = User::new(
        "id".into(),
        "name".into(),
        "password".into(),
        "email".into(),
        18,
        friends.clone(),
    );

    assert_eq!(*user_new.get_age(), 18);
    assert_eq!(*user_new.get_email(), "email");
    assert_eq!(*user_new.get_password(), "password");
    assert_eq!(*user_new.get_id(), "id");
    assert_eq!(*user_new.get_name(), "name");
    assert_eq!(*user_new.get_friends(), friends.clone());
    assert_eq!(*user_default.get_age(), 0);
    assert_eq!(*user_default.get_email(), "");
    assert_eq!(*user_default.get_password(), "");
    assert_eq!(*user_default.get_id(), "");
    assert_eq!(*user_default.get_name(), "");
    assert_eq!(*user_default.get_friends(), vec![]);
    assert_eq!(*user_new.get_mut_age(), 18);
    assert_eq!(*user_new.get_mut_email(), "email");
    assert_eq!(*user_new.get_mut_password(), "password");
    assert_eq!(*user_new.get_mut_id(), "id");
    assert_eq!(*user_new.get_mut_name(), "name");
    assert_eq!(*user_new.get_mut_friends(), friends.clone());
    assert_eq!(*user_default.get_mut_age(), 0);
    assert_eq!(*user_default.get_mut_email(), "");
    assert_eq!(*user_default.get_mut_password(), "");
    assert_eq!(*user_default.get_mut_id(), "");
    assert_eq!(*user_default.get_mut_name(), "");
    assert_eq!(*user_default.get_mut_friends(), vec![]);

    user_new.set_age(12);
    user_new.set_email("new email");
    user_new.set_password("new password");
    user_new.set_name("new name");
    user_new.set_id("new id");
    user_new.get_mut_friends().iter_mut().enumerate().for_each(
        |(index, friend)| {
            let enumerate = index as u8;
            let email = format!("{}-email@gmail.com", enumerate);
            let name = format!("{}-user", enumerate);
            let password = format!("{}-password", enumerate);
            let age = enumerate * 2;
            friend.set_age(age);
            friend.set_id(enumerate.to_string());
            friend.set_email(email.clone());
            friend.set_name(name.clone());
            friend.set_password(password.clone());
            assert_eq!(*friend.get_age(), age);
            assert_eq!(*friend.get_email(), email);
            assert_eq!(*friend.get_name(), name);
            assert_eq!(*friend.get_password(), password);
            assert_eq!(*friend.get_id(), enumerate.to_string());
            assert_eq!(*friend.get_mut_age(), age);
            assert_eq!(*friend.get_mut_email(), email);
            assert_eq!(*friend.get_mut_name(), name);
            assert_eq!(*friend.get_mut_password(), password);
            assert_eq!(*friend.get_mut_id(), enumerate.to_string());
        },
    );
    user_default.set_age(12);
    user_default.set_email("new email");
    user_default.set_password("new password");
    user_default.set_name("new name");
    user_default.set_id("new id");
    user_default.set_friends(user_new.get_friends().clone());

    assert_eq!(*user_new.get_age(), 12);
    assert_eq!(*user_new.get_email(), "new email");
    assert_eq!(*user_new.get_password(), "new password");
    assert_eq!(*user_new.get_id(), "new id");
    assert_eq!(*user_new.get_name(), "new name");
    assert_eq!(*user_new.get_mut_age(), 12);
    assert_eq!(*user_new.get_mut_email(), "new email");
    assert_eq!(*user_new.get_mut_password(), "new password");
    assert_eq!(*user_new.get_mut_id(), "new id");
    assert_eq!(*user_new.get_mut_name(), "new name");
    assert_eq!(*user_default.get_age(), 12);
    assert_eq!(*user_default.get_email(), "new email");
    assert_eq!(*user_default.get_password(), "new password");
    assert_eq!(*user_default.get_id(), "new id");
    assert_eq!(*user_default.get_name(), "new name");
    assert_eq!(*user_default.get_mut_age(), 12);
    assert_eq!(*user_default.get_mut_email(), "new email");
    assert_eq!(*user_default.get_mut_password(), "new password");
    assert_eq!(*user_default.get_mut_id(), "new id");
    assert_eq!(*user_default.get_mut_name(), "new name");

    // getters mut setters
    *user_new.get_mut_age() = 24;
    *user_new.get_mut_email() = "mut email".to_string();
    *user_new.get_mut_password() = "mut password".to_string();
    *user_new.get_mut_name() = "mut name".to_string();
    *user_new.get_mut_id() = "mut id".to_string();
    *user_default.get_mut_age() = 24;
    *user_default.get_mut_email() = "mut email".to_string();
    *user_default.get_mut_password() = "mut password".to_string();
    *user_default.get_mut_name() = "mut name".to_string();
    *user_default.get_mut_id() = "mut id".to_string();

    // after getters mut setters, getters retest
    assert_eq!(*user_new.get_age(), 24);
    assert_eq!(*user_new.get_email(), "mut email");
    assert_eq!(*user_new.get_password(), "mut password");
    assert_eq!(*user_new.get_id(), "mut id");
    assert_eq!(*user_new.get_name(), "mut name");
    assert_eq!(*user_new.get_mut_age(), 24);
    assert_eq!(*user_new.get_mut_email(), "mut email");
    assert_eq!(*user_new.get_mut_password(), "mut password");
    assert_eq!(*user_new.get_mut_id(), "mut id");
    assert_eq!(*user_new.get_mut_name(), "mut name");
    assert_eq!(*user_default.get_age(), 24);
    assert_eq!(*user_default.get_email(), "mut email");
    assert_eq!(*user_default.get_password(), "mut password");
    assert_eq!(*user_default.get_id(), "mut id");
    assert_eq!(*user_default.get_name(), "mut name");
    assert_eq!(*user_default.get_mut_age(), 24);
    assert_eq!(*user_default.get_mut_email(), "mut email");
    assert_eq!(*user_default.get_mut_password(), "mut password");
    assert_eq!(*user_default.get_mut_id(), "mut id");
    assert_eq!(*user_default.get_mut_name(), "mut name");
}
