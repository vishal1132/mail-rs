use crate::email::EmailProviders;

pub struct User {
    pub fname: String,
    pub lname: String,
    pub display_name: String,
    pub email: String,
    pub email_provider: EmailProviders,
    pub password: String,
}

impl User {
    pub fn str(&self) -> String {
        format!(
            "{},{},{},{},{},{}",
            self.fname,
            self.lname,
            self.display_name,
            self.email,
            self.email_provider,
            self.password
        )
    }

    pub fn from_str(s: String) -> Self {
        let mut iter = s.split(',');
        User {
            fname: iter.next().unwrap().to_string(),
            lname: iter.next().unwrap().to_string(),
            display_name: iter.next().unwrap().to_string(),
            email: iter.next().unwrap().to_string(),
            email_provider: match iter.next().unwrap() {
                "gmail" => EmailProviders::Gmail,
                _ => EmailProviders::Gmail,
            },
            password: iter.next().unwrap().to_string(),
        }
    }
}
