use lettre::{Message, message::header::ContentType, transport::smtp::authentication::Credentials, SmtpTransport};

use super::darth_tools::DarthTools;

pub trait LettreTrait {
    /// Sends an email using the SMTP protocol through the 'lettre' library.
    ///
    /// # Arguments
    /// - `from_name`: Sender's name.
    /// - `from_email`: Sender's email address.
    /// - `to_name`: Recipient's name.
    /// - `to_email`: Recipient's email address.
    /// - `user_smtp`: SMTP server's username.
    /// - `password_smtp`: SMTP server's password.
    /// - `provider`: Email provider (e.g., "smtp.gmail.com").
    /// - `subject`: Email subject.
    /// - `body`: Email body.
    fn new_lettre_send_email(
        from_name: &str,
        from_email: &str,
        to_name: &str,
        to_email: &str,
        user_smtp: &str,
        password_smtp: &str,
        provider: &str,
        subject: &str,
        body: &str,
    ) -> Result<(), lettre::error::Error>;
}

impl LettreTrait for DarthTools {
    fn new_lettre_send_email(
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
}