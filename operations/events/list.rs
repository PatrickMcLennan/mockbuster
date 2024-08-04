use models::events_list_result::EventsListResult;
use sea_orm::{
    prelude::*, DatabaseBackend, DatabaseConnection, FromQueryResult, JsonValue, Statement,
};

const LOG_KEY: &str = "[Operations::Events::List]: ";

// Fetch the 20 latest comments or ratings and return each as an "Event".
// An event can contain a comment and/or a rating, along with the user that
// created it.
// For good UX, if a user leaves a rating and comment on the same movie within 24hrs,
// aggregate them into 1 event.

pub async fn execute(db: DatabaseConnection) -> Result<Vec<EventsListResult>, DbErr> {
    match JsonValue::find_by_statement(Statement::from_sql_and_values(
            DatabaseBackend::Postgres,
            "
            WITH latest_comments AS (
                SELECT 
                    'comment' AS event_type,
                    c.user_id, 
                    c.tmdb_id, 
                    c.created_at, 
                    c.id AS event_id,
                    row_to_json(c) AS comment_json,
                    NULL::json AS rating_json,
                    row_to_json(u) AS user_json
                FROM 
                    comments c
                JOIN 
                    users u ON c.user_id = u.id
                ORDER BY 
                    c.created_at DESC
                LIMIT 100
            ),
            latest_ratings AS (
                SELECT 
                    'rating' AS event_type,
                    r.user_id, 
                    r.tmdb_id, 
                    r.created_at, 
                    r.id AS event_id,
                    NULL::json AS comment_json,
                    row_to_json(r) AS rating_json,
                    row_to_json(u) AS user_json
                FROM 
                    ratings r
                JOIN 
                    users u ON r.user_id = u.id
                ORDER BY 
                    r.created_at DESC
                LIMIT 100
            ),
            latest_events AS (
                SELECT * FROM latest_comments
                UNION ALL
                SELECT * FROM latest_ratings
                ORDER BY created_at DESC
                LIMIT 100
            ),
            combined_events AS (
                SELECT
                    le.user_id,
                    le.tmdb_id,
                    le.created_at,
                    COALESCE(lc.comment_json, le.comment_json) AS comment_json,
                    COALESCE(lr.rating_json, le.rating_json) AS rating_json,
                    le.user_json
                FROM 
                    latest_events le
                LEFT JOIN latest_comments lc
                    ON le.user_id = lc.user_id 
                    AND le.tmdb_id = lc.tmdb_id 
                    AND lc.created_at BETWEEN le.created_at - interval '24 hours' AND le.created_at + interval '24 hours'
                LEFT JOIN latest_ratings lr
                    ON le.user_id = lr.user_id 
                    AND le.tmdb_id = lr.tmdb_id 
                    AND lr.created_at BETWEEN le.created_at - interval '24 hours' AND le.created_at + interval '24 hours'
            ),
            distinct_events AS (
                SELECT DISTINCT ON (ce.user_id, ce.tmdb_id, ce.created_at)
                    ce.user_id,
                    ce.tmdb_id,
                    ce.created_at,
                    ce.comment_json,
                    ce.rating_json,
                    ce.user_json
                FROM 
                    combined_events ce
                ORDER BY 
                    ce.user_id, 
                    ce.tmdb_id, 
                    ce.created_at DESC
            )
            SELECT 
                json_build_object(
                    'comment', comment_json,
                    'rating', rating_json,
                    'user', user_json
                ) AS event_json
            FROM 
                distinct_events
            ORDER BY 
                created_at DESC
            LIMIT 20;
            ",
            [],
        ))
        .all(&db)
        .await
    {
        Ok(records) => {
            Ok(records.into_iter().filter_map(|value| {
                let json = &value["event_json"];
                match serde_json::from_value::<EventsListResult>(json.clone()) {
                    Ok(v) => Some(v),
                    Err(e) => {
                        println!("{}{:?}", LOG_KEY, e);
                        None
                    }
                }
            }).collect::<Vec<EventsListResult>>())
        },
        Err(e) => {
            println!("{}{:?}", LOG_KEY, e);
            Err(e)
        }
    }
}
