use crate::email::Emailer;

const GMAIL_SMTP_ADDRESS: &str = "smtp.gmail.com";

pub struct Gmail;

impl Emailer for Gmail {
    fn smtp_address() -> String {
        GMAIL_SMTP_ADDRESS.to_string()
    }
}
