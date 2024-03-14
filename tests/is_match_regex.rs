mod config;
pub use config::User;

#[test]
fn is_match_regex() {
    let user = User::new(
        "123".to_string(),
        "John Doe".to_string(),
        "password123".to_string(),
        "john.doe@example.com".to_string(),
        30,
        vec![],
    );

    assert!(user.is_match_id_regex(r"^\d+$").is_ok());
    assert!(user.is_match_name_regex(r"^[a-zA-Z\s]+$").is_ok());
    assert!(user.is_match_password_regex(r"^[a-zA-Z0-9]+$").is_ok());
    assert!(user
        .is_match_email_regex(
            r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$"
        )
        .is_ok());
}
