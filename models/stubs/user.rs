use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Clone, Debug, Deserialize, PartialEq, Properties, Serialize)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub password_hash: String,
    pub email: String,
    pub permission: i32,
    pub created_at: String,
    pub updated_at: String,
}
