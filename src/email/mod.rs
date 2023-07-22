pub mod gmail;
use crate::user::User;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use strum_macros::Display;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Display)]
pub enum EmailProviders {
    #[strum(serialize = "gmail")]
    Gmail,
}

pub fn get_email_provider(s: &str) -> EmailProviders {
    match s {
        "gmail" => EmailProviders::Gmail,
        _ => EmailProviders::Gmail,
    }
}

pub trait Emailer {
    fn smtp_address() -> String;
}


pub struct emailer {
    mailer: lettre::SmtpTransport,
    u: User,
}

impl emailer {
    pub fn new(e: EmailProviders, u: crate::user::User) -> Self {
        let smtp_address = match e {
            EmailProviders::Gmail => gmail::Gmail::smtp_address(),
        };
        let creds = Credentials::new(u.email.clone(), u.password.clone());
        let mailer = SmtpTransport::relay(smtp_address.as_str())
            .unwrap()
            .credentials(creds)
            .build();

        emailer { mailer, u }
    }

    pub fn send_mail(&self, body: &str, subject: &str, to: &str) -> anyhow::Result<()> {
        let email = Message::builder()
            .from(
                format!("{} <{}>", self.u.display_name, self.u.email)
                    .parse()
                    .unwrap(),
            )
            .to(to.parse().unwrap())
            .subject(subject)
            .header(ContentType::TEXT_PLAIN)
            .body(body.to_string())
            .unwrap();

        let _ = self.mailer.send(&email)?;
        Ok(())
    }
}
