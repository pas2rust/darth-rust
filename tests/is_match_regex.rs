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
    assert!(user.is_match_regex_id(r"^\d+$").is_ok());
    assert!(user.is_match_regex_name(r"^[a-zA-Z\s]+$").is_ok());
    assert!(user.is_match_regex_password(r"^[a-zA-Z0-9]+$").is_ok());
    assert!(user
        .is_match_regex_email(
            r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$"
        )
        .is_ok());
}
