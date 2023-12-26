use db_models::generated::{ratings, users};
use sea_orm::{prelude::*, DatabaseConnection, QueryOrder, QuerySelect};
// use serde::{Deserialize, Serialize};
// use validator::Validate;
use validators::recently_rented_dto::RecentlyRentedDTO;

pub async fn get_recently_rented_movies(
    pagination: u64,
    db: DatabaseConnection,
) -> Vec<(ratings::Model, Vec<users::Model>)> {
    let offset = (pagination - 1) * 20;

    let postgres = match ratings::Entity::find()
        .find_with_related(users::Entity)
        .group_by(ratings::Column::Id)
        .group_by(users::Column::Id)
        .order_by_asc(ratings::Column::CreatedAt)
        .limit(Some(20 as u64))
        .offset(offset)
        .all(&db)
        .await
    {
        Ok(v) => v,
        Err(e) => {
            println!("[get_recently_rented_movies]: {:?}", e);
            vec![]
        }
    };

	postgres
}
