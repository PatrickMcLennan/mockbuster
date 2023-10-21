use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::Validate;

#[derive(Debug, Default, Clone, Serialize, Deserialize, Validate)]
pub struct SearchDTO {
    #[validate(length(min = 1, message = "No search term entered"))]
    pub query: String,
    #[validate(range(min = 1, message = "Page must be a value > 1"))]
    pub page: i64,
}

impl SearchDTO {
    pub fn to_json(&self) -> String {
        serde_json::to_string(&json!({
            "query": self.query,
            "page": self.page,
        }))
        .unwrap()
    }

    pub fn from_json(json_str: &str) -> Result<SearchDTO, serde_json::Error> {
        serde_json::from_str(json_str)
    }

    pub fn get_errors(&self) -> Option<SearchDTO> {
        match self.validate() {
            Ok(_) => None,
            Err(e) => {
                let mut query_error = String::new();

                for error in e.field_errors() {
                    let field = error.0.to_string();
                    let message = error
                        .1
                        .first()
                        .expect("Invalid value")
                        .message
                        .clone()
                        .expect("Invalid value");

                    if field == "query" {
                        query_error = message.to_string()
                    };
                }

                Some(SearchDTO {
                    query: query_error,
                    page: 0,
                })
            }
        }
    }
}
