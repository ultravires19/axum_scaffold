use validator::Validate;

#[derive(Clone, Debug, Validate)]
pub struct SubscriberEmail {
    #[validate(email)]
    email: String,
}

impl SubscriberEmail {
    pub fn parse(s: String) -> Result<Self, String> {
        let subscriber_email = Self { email: s };
        match subscriber_email.validate() {
            Ok(_) => Ok(subscriber_email),
            Err(_) => Err(format!("not a valid email address")),
        }
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
