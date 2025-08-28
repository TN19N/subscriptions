mod email;
mod name;

use crate::handlers::FormData;
use email::SubscriberEmail;
use name::SubscriberName;
use validator::ValidationErrors;

#[derive(Debug)]
pub struct Subscriber {
    pub email: SubscriberEmail,
    pub name: SubscriberName,
}

impl TryFrom<FormData> for Subscriber {
    type Error = validator::ValidationErrors;

    fn try_from(value: FormData) -> Result<Self, Self::Error> {
        let mut errors = ValidationErrors::new();

        let email: Option<SubscriberEmail> = match value.email.try_into() {
            Ok(email) => Some(email),
            Err(error) => {
                errors.add("email", error);
                None
            }
        };

        let name: Option<SubscriberName> = match value.name.try_into() {
            Ok(name) => Some(name),
            Err(error) => {
                errors.add("name", error);
                None
            }
        };

        if let Some(email) = email
            && let Some(name) = name
            && errors.is_empty()
        {
            Ok(Self { email, name })
        } else {
            Err(errors)
        }
    }
}
