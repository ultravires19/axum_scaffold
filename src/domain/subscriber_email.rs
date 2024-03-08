use validator::{Validate, ValidationErrors};

#[derive(Debug, Validate)]
pub struct SubscriberEmail {
    #[validate(email)]
    email: String,
}

impl SubscriberEmail {
    pub fn parse(s: String) -> Result<Self, ValidationErrors> {
        let subscriber_email = Self { email: s };
        subscriber_email.validate()?;
        Ok(subscriber_email)
    }
}

impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.email
    }
}

#[cfg(test)]
mod tests {
    use super::SubscriberEmail;
    use claims::assert_err;

    #[test]
    fn empty_string_is_rejected() {
        let email = "".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn email_missing_arroba_rejected() {
        let email = "nomamesgueydot.com".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn email_missing_subject_rejected() {
        let email = "@yeemail.com".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }
}
