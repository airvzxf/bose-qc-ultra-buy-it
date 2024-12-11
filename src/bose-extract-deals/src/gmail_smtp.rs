use crate::errors::AppError;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::env;

pub fn send_email(from: &str, to: &str, subject: &str, body: &str) -> Result<(), AppError> {
    let email = Message::builder()
        .from(
            from.parse::<lettre::Address>()
                .map_err(|e| AppError::General(e.to_string()))?
                .into(),
        )
        .to(to
            .parse::<lettre::Address>()
            .map_err(|e| AppError::General(e.to_string()))?
            .into())
        .subject(subject)
        .header(ContentType::TEXT_PLAIN)
        .body(String::from(body))
        .map_err(|e| AppError::General(e.to_string()))?;

    let email_address = env::var("GMAIL_ADDRESS")
        .map_err(|_| AppError::General("GMAIL_ADDRESS environment variable not set".to_string()))?;
    let email_password = env::var("GMAIL_PASSWORD").map_err(|_| {
        AppError::General("GMAIL_PASSWORD environment variable not set".to_string())
    })?;

    let creds = Credentials::new(email_address, email_password);

    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .map_err(|e| AppError::General(e.to_string()))?
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => Ok(()),
        Err(e) => Err(AppError::General(format!("Failed to send email: {}", e))),
    }
}
