use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport};

pub fn new_smtp_lettre_send_email(
    from_name: &str,
    from_email: &str,
    to_name: &str,
    to_email: &str,
    user_smtp: &str,
    password_smtp: &str,
    provider: &str,
    subject: &str,
    body: &str,
) -> Result<(), lettre::error::Error> {
    let from = format!("{} <{}>", from_name, from_email);
    let to: String = format!("{} <{}>", to_name, to_email);
    Message::builder()
        .from(from.parse().expect("error in from"))
        .reply_to(from.parse().expect("error in reply_to"))
        .to(to.parse().expect("error in to"))
        .subject(subject)
        .header(ContentType::TEXT_HTML)
        .body(body.to_string())?;
    let credentials = Credentials::new(user_smtp.to_owned(), password_smtp.to_owned());
    SmtpTransport::relay(provider)
        .expect("transport error")
        .credentials(credentials)
        .build();
    Ok(())
}
