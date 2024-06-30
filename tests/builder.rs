#![cfg(all(feature = "build", feature = "json"))]
use darth_rust::DarthRust;
use serde::{Deserialize, Serialize};

#[derive(
    DarthRust,
    Deserialize,
    Serialize,
    Debug,
    PartialEq,
    Clone,
)]
pub struct User {
    #[pattern(r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[1-5][0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12}$")]
    pub id: String,
    #[pattern(r"^[a-zA-Z]{3,20}\s[a-zA-Z]{3,20}$")]
    pub name: String,
    #[pattern(r"^[a-zA-Z]{6,20}")]
    pub password: String,
    #[pattern(
        r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$"
    )]
    #[pattern_notify("email must be valid")]
    pub email: String,
    #[min(18)]
    #[min_notify("must be over 18 years old")]
    #[max(30)]
    #[min_notify("must be at most 30 years old")]
    pub age: u8,
    pub friends: Vec<User>,
}

#[test]
fn builder() {
    let user = User::new()
        .id("123e4567-e89b-12d3-a456-426614174000")
        .name("John Doe")
        .password("password123")
        .email("johndoe@example.com")
        .age(18)
        .friends(vec![])
        .build()
        .unwrap();

    assert_eq!(
        user.id,
        "123e4567-e89b-12d3-a456-426614174000"
    );
    assert_eq!(user.name, "John Doe");
    assert_eq!(user.password, "password123");
    assert_eq!(user.email, "johndoe@example.com");
    assert_eq!(user.age, 18);
    assert_eq!(user.friends, vec![]);
}
