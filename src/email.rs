use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

use crate::personal_info::{gmail_user, gmail_pass};

pub struct Email {
    name: String,
    at: String,
    host: String,
}

impl Email {
    pub fn new(name: &str, at: &str, host: &str) -> Email {
        Email { name: name.to_string(), 
            at: at.to_string(), 
            host: host.to_string()
        }
    }

    //TODO: Send HTML
    pub fn send_to(&self, recipient: &str) {
        let from = format!("{} <{}@{}>", self.name, self.at, self.host);
        let to = format!("Friend <{}>", recipient);
        println!("email: {}", from);
        let email = Message::builder()
            .from(from.as_str().parse().unwrap())
            .to(to.as_str().parse().unwrap())
            .subject("Thank you for signing up!")
            .body(String::from(":heart_eyes:"))
            .unwrap();

       let creds = Credentials::new(gmail_user.to_string(), gmail_pass.to_string());

        // Open a remote connection to gmail
        let mailer = SmtpTransport::relay("smtp.gmail.com")
            .unwrap()
            .credentials(creds)
            .build(); 
        
        // let mailer = SmtpTransport::unencrypted_localhost();
        assert!(mailer.send(&email).is_ok());
    }
}

