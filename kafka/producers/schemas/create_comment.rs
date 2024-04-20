use serde_json::json;
use serde_json::Error;
use serde_json::Value;

const LOG_KEY: &str = "[Producers::Schemas::create_comment]";

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct CreateCommentSchema {
    pub id: i32,
    pub tmdb_id: i32,
    pub user_id: i32,
}

impl CreateCommentSchema {
    pub fn to_json(&self) -> Value {
        json!({
            "id": self.id,
            "tmdb_id": self.tmdb_id,
            "user_id": self.user_id,
        })
    }

    pub fn from_json(json: &str) -> Result<Self, Error> {
        match serde_json::from_str(json) {
            Ok(v) => Ok(v),
            Err(e) => {
                println!("{}: {:?}", LOG_KEY, e);
                Err(e)
            }
        }
    }
}
