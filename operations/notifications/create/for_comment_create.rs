use models::generated::{notifications, subscriptions, users};
use sea_orm::{prelude::*, ActiveValue::Set, DatabaseConnection, Statement};

const LOG_KEY: &str = "[Operations::Notifications::Create::ForCommentCreate]: ";

pub async fn execute(
    users: Vec<users::Model>,
    db: DatabaseConnection,
    comment_id: i32,
) -> Result<i32, DbErr> {
    let user_iter = users.into_iter();

    let last_inserted_notification =
        match notifications::Entity::insert_many(user_iter.clone().map(|user| {
            notifications::ActiveModel {
                user_id: Set(user.id),
                notification_type: Set(1),
                seen: Set(false),
                seen_at: Set(None),
                related_id: Set(comment_id),
                ..Default::default()
            }
        }))
        .exec(&db)
        .await
        {
            Ok(r) => r.last_insert_id,
            Err(e) => {
                println!("{} {}", LOG_KEY, e);
                return Err(e);
            }
        };

    let subscriptions = match subscriptions::Entity::find()
        .from_raw_sql(Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            "
                SELECT *
                FROM subscriptions
                WHERE user_id IN $1;
            ",
            [user_iter.map(|user| user.id).collect::<Vec<i32>>().into()],
        ))
        .all(&db)
        .await
    {
        Ok(v) => v,
        Err(e) => {
            println!("{} {}", LOG_KEY, e);
            return Err(e);
        }
    };

    return Ok(last_inserted_notification);
}
