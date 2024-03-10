use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::Validate;

#[derive(Debug, Default, Clone, Serialize, Deserialize, Validate, PartialEq)]
pub struct GetProfileDto {
    #[validate(range(min = 1, message = "No profile id found"))]
    pub id: i32,
}
impl GetProfileDto {
    pub fn to_json(&self) -> String {
        serde_json::to_string(&json!({
            "id": self.id,
        }))
        .unwrap()
    }

    pub fn from_json(json_str: &str) -> Result<GetProfileDto, serde_json::Error> {
        serde_json::from_str(json_str)
    }

    pub fn get_errors(&self) -> Option<GetProfileDto> {
        match self.validate() {
            Ok(_) => None,
            Err(_) => Some(GetProfileDto { id: -1 }),
        }
    }
}
