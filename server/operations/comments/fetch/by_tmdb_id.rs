use models::generated::{comments, users};
use sea_orm::{prelude::*, DatabaseConnection, QueryOrder};

const LOG_KEY: &str = "[Operations::Comments::Fetch::ByTmdbId]: ";

pub async fn execute(
    tmdb_id: u32,
    db: DatabaseConnection,
) -> Result<Vec<(comments::Model, Option<users::Model>)>, DbErr> {
    match comments::Entity::find()
        .order_by_desc(comments::Column::CreatedAt)
        .filter(comments::Column::TmdbId.eq(tmdb_id))
        .find_also_related(users::Entity)
        .all(&db)
        .await
    {
        Ok(comments) => Ok(comments),
        Err(e) => {
            return {
                println!("{}{:?}", LOG_KEY, e);
                Err(e)
            }
        }
    }
}
