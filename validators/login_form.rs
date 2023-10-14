use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::{Validate, ValidationError};

fn validate_password(password: &str) -> Result<(), ValidationError> {
    let uppercase_regex = Regex::new(r"[A-Z]").unwrap();
    let lowercase_regex = Regex::new(r"[a-z]").unwrap();
    let digit_regex = Regex::new(r"\d").unwrap();
    let special_regex = Regex::new(r"[^A-Za-z\d\s]").unwrap();

    if uppercase_regex.is_match(password)
        && lowercase_regex.is_match(password)
        && digit_regex.is_match(password)
        && special_regex.is_match(password)
        && password.len() >= 8
        && password.len() <= 12
    {
        Ok(())
    } else {
        Err(ValidationError::new("password"))
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, Validate)]
pub struct LoginFormSchema {
    #[validate(email(message = "Must be a valid email"))]
    pub email: String,
    #[validate(custom(
        function = "validate_password",
        message = "A valid password must be 8 - 12 characters, with minimum 1 uppercase, lowercase, digit & special character each."
    ))]
    pub password: String,
}

impl LoginFormSchema {
    pub fn to_json(&self) -> String {
        serde_json::to_string(&json!({
            "email": self.email,
            "password": self.password,
        }))
        .unwrap()
    }

    pub fn from_json(json_str: &str) -> Result<LoginFormSchema, serde_json::Error> {
        serde_json::from_str(json_str)
    }

    pub fn get_errors(&self) -> Option<LoginFormSchema> {
        match self.validate() {
            Ok(_) => None,
            Err(e) => {
                let mut email_error = String::new();
                let mut password_error = String::new();

                for error in e.field_errors() {
                    let field = error.0.to_string();
                    let message = error
                        .1
                        .first()
                        .expect("Invalid value")
                        .message
                        .clone()
                        .expect("Invalid value");

                    if field == "email" {
                        email_error = message.to_string()
                    };
                    if field == "password" {
                        password_error = message.to_string()
                    };
                }

                Some(LoginFormSchema {
                    email: email_error,
                    password: password_error,
                })
            }
        }
    }
}
