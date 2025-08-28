use validator::{ValidateEmail, ValidationError};

#[derive(Debug)]
pub struct SubscriberEmail(String);

impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for SubscriberEmail {
    type Error = validator::ValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.validate_email() {
            true => Ok(Self(value)),
            false => {
                let mut error = ValidationError::new("Invalid Subscriber Email");
                error.add_param("Invalid Email".into(), &value);
                Err(error)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SubscriberEmail;
    use claims::assert_err;
    use claims::assert_ok;
    use fake::Fake;
    use fake::faker::internet::en::SafeEmail;
    use proptest::prelude::*;

    #[test]
    fn empty_string_is_rejected() {
        let email = "".to_string();
        assert_err!(SubscriberEmail::try_from(email));
    }

    #[test]
    fn email_missing_at_symbol_is_rejected() {
        let email = "ursuladomain.com".to_string();
        assert_err!(SubscriberEmail::try_from(email));
    }

    #[test]
    fn email_missing_subject_is_rejected() {
        let email = "@domain.com".to_string();
        assert_err!(SubscriberEmail::try_from(email));
    }

    prop_compose! {
        fn valid_email_strategy()(email in any::<()>().prop_map(|_| SafeEmail().fake::<String>())) -> String {
            email.to_string()
        }
    }

    proptest! {
        #[test]
        fn test_valid_emails_are_accepted(email in valid_email_strategy()) {
            dbg!(&email);

            assert_ok!(SubscriberEmail::try_from(email));
        }
    }
}
