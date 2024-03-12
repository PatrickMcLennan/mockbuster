use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[cfg(not(feature = "ssr"))]
#[derive(Clone, Debug, Deserialize, PartialEq, Properties, Serialize)]
pub struct Rating {
    pub id: f32,
    pub user_id: f32,
    pub score: f32,
    pub tmdb_id: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
