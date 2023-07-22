use crate::email::Emailers;

const GMAIL_SMTP_ADDRESS: &str = "smtp.gmail.com";

pub struct Gmail;

impl Emailers for Gmail {
    fn smtp_address() -> String {
        GMAIL_SMTP_ADDRESS.to_string()
    }
}
