mod config;
pub use config::User;

#[test]
fn test_user_builder() {
    let user = User::builder()
        .id("123")
        .name("John Doe")
        .password("password123")
        .email("johndoe@example.com")
        .age(30)
        .friends(vec![])
        .build()
        .unwrap();

    assert_eq!(user.id, "123");
    assert_eq!(user.name, "John Doe");
    assert_eq!(user.password, "password123");
    assert_eq!(user.email, "johndoe@example.com");
    assert_eq!(user.age, 30);
    assert_eq!(user.friends, vec![]);
}
