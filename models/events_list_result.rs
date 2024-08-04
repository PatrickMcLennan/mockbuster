use crate::generated::{comments, ratings, users};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct EventsListResult {
    pub comment: Option<comments::Model>,
    pub rating: Option<ratings::Model>,
    pub user: users::Model,
}
