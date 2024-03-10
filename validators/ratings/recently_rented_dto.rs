use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct RecentlyRentedDTO {
    pub page: Option<u64>,
}

impl RecentlyRentedDTO {
    pub fn to_json(&self) -> String {
        serde_json::to_string(&json!({
            "page": self.page,
        }))
        .unwrap()
    }

    pub fn from_json(json_str: &str) -> Result<RecentlyRentedDTO, serde_json::Error> {
        serde_json::from_str(json_str)
    }
}
