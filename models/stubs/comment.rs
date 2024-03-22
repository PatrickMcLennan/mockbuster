use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[cfg(not(feature = "ssr"))]
#[derive(Clone, Debug, Deserialize, PartialEq, Properties, Serialize)]
pub struct CommentStub {
    pub id: i32,
    pub user_id: i32,
    pub content: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub tmdb_id: i32,
}
