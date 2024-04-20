use models::generated::users;
use sea_orm::{prelude::*, DatabaseBackend, DatabaseConnection, Statement};

const LOG_KEY: &str = "[Operations::Users::List::ByNewCommentNotification]: ";

// Get every other user that
//     1. hasn't ingored comments from all or this user,
//     2. has already commented on this movie

pub async fn execute(
    db: DatabaseConnection,
    comment_creator_id: i32,
    tmdb_id: i32,
) -> Result<Vec<users::Model>, DbErr> {
    match users::Entity::find()
        .from_raw_sql(Statement::from_sql_and_values(
            DatabaseBackend::Postgres,
            "
                SELECT users.* 
                FROM users
                JOIN user_preference AS preference ON preference.user_id = users.id
                WHERE users.id != $1 
                AND preference.allow_comments IS TRUE
                AND (preference.ignored_user_comments IS NULL OR NOT $1 = ANY(preference.ignored_user_comments))
                AND EXISTS (
                    SELECT 1
                    FROM comments
                    WHERE comments.user_id = $1
                    AND comments.tmdb_id = $2
                    HAVING COUNT(*) >= 1
                );
            ",
            [comment_creator_id.into(), tmdb_id.into()],
        ))
        .all(&db)
        .await
    {
        Ok(records) => Ok(records),
        Err(e) => {
            println!("{}{:?}", LOG_KEY, e);
            Err(e)
        }
    }
}
