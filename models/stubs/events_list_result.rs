use crate::{
    stubs::{comment::Comment, rating::Rating, user::User},
    tmdb_movies::movie_id_result,
};
use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[cfg(not(feature = "ssr"))]
#[derive(Clone, Debug, Deserialize, PartialEq, Properties, Serialize)]
pub struct EventsListResult {
    pub comment: Option<Comment>,
    pub rating: Option<Rating>,
    pub user: User,
    pub tmdb_movie: Option<movie_id_result::MovieIdResult>,
}
