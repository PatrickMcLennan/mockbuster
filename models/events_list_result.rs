use crate::{
    generated::{comments, ratings, users},
    tmdb_movies::movie_id_result,
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct EventsListResult {
    pub comment: Option<comments::Model>,
    pub rating: Option<ratings::Model>,
    pub user: users::Model,
    pub tmdb_movie: Option<movie_id_result::MovieIdResult>,
}
