use darth_rust::DarthRust;
#[derive(DarthRust, Debug, PartialEq)]
struct User {
    id: String,
    name: String,
    password: String,
    email: String,
    age: u8,
}

#[test]
pub fn getters_and_setters() {
    let mut user_default = User::default();
    let mut user_new = User::new(
        "id".to_string(),
        "name".to_string(),
        "password".to_string(),
        "email".to_string(),
        18,
    );
    // prints in case error
    user_new.print_err();
    user_default.print_err();

    // getters new/default test
    assert_eq!(*user_new.get_age(), 18);
    assert_eq!(*user_new.get_email(), "email");
    assert_eq!(*user_new.get_password(), "password");
    assert_eq!(*user_new.get_id(), "id");
    assert_eq!(*user_new.get_name(), "name");
    assert_eq!(*user_default.get_age(), 0);
    assert_eq!(*user_default.get_email(), "");
    assert_eq!(*user_default.get_password(), "");
    assert_eq!(*user_default.get_id(), "");
    assert_eq!(*user_default.get_name(), "");
    assert_eq!(*user_new.get_mut_age(), 18);
    assert_eq!(*user_new.get_mut_email(), "email");
    assert_eq!(*user_new.get_mut_password(), "password");
    assert_eq!(*user_new.get_mut_id(), "id");
    assert_eq!(*user_new.get_mut_name(), "name");
    assert_eq!(*user_default.get_mut_age(), 0);
    assert_eq!(*user_default.get_mut_email(), "");
    assert_eq!(*user_default.get_mut_password(), "");
    assert_eq!(*user_default.get_mut_id(), "");
    assert_eq!(*user_default.get_mut_name(), "");

    //setters test
    user_new.set_age(12);
    user_new.set_email("new email");
    user_new.set_password("new password");
    user_new.set_name("new name");
    user_new.set_id("new id");
    user_default.set_age(12);
    user_default.set_email("new email");
    user_default.set_password("new password");
    user_default.set_name("new name");
    user_default.set_id("new id");

    // after setters, getters retest
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

    //getters mut setters
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
