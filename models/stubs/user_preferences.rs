use serde::Serialize;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties, Serialize)]
pub struct Model {
    pub id: i32,
    pub user_id: i32,
    pub allow_comments: bool,
    pub allow_ratings: bool,
    pub ignored_user_ratings: Option<Vec<i64>>,
    pub ignored_user_comments: Option<Vec<i64>>,
}
