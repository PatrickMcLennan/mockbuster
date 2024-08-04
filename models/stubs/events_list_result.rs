use crate::stubs::{comment::Comment, rating::Rating, user::User};
use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[cfg(not(feature = "ssr"))]
#[derive(Clone, Debug, Deserialize, PartialEq, Properties, Serialize)]
pub struct EventsListResult {
    pub comment: Option<Comment>,
    pub rating: Option<Rating>,
    pub user: User,
}
