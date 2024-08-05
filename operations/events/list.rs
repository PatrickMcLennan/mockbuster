use crate::tmdb_movies;
use models::{events_list_result::EventsListResult, tmdb_movies::movie_id_result::MovieIdResult};
use sea_orm::{
    prelude::*, DatabaseBackend, DatabaseConnection, FromQueryResult, JsonValue, Statement,
};
use std::collections::HashMap;

const LOG_KEY: &str = "[Operations::Events::List]: ";

// Fetch the 20 latest comments or ratings and return each as an "Event".
// An event can contain a comment and/or a rating, along with the user that
// created it.

// For good UX, if a user leaves a rating and comment on the same movie within 24hrs,
// aggregate them into 1 event.

pub async fn execute(
    db: DatabaseConnection,
    http_client: reqwest_middleware::ClientWithMiddleware,
) -> Result<Vec<EventsListResult>, DbErr> {
    match JsonValue::find_by_statement(Statement::from_sql_and_values(
        DatabaseBackend::Postgres,
        "
        WITH latest_comments AS (
            SELECT 
                c.user_id, 
                c.tmdb_id, 
                c.created_at AS comment_created_at,
                NULL::timestamp AS rating_created_at,
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
                r.user_id, 
                r.tmdb_id, 
                NULL::timestamp AS comment_created_at,
                r.created_at AS rating_created_at,
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
            SELECT 
                user_id, 
                tmdb_id, 
                comment_created_at, 
                rating_created_at, 
                comment_json, 
                rating_json, 
                user_json 
            FROM latest_comments
            UNION ALL
            SELECT 
                user_id, 
                tmdb_id, 
                comment_created_at, 
                rating_created_at, 
                comment_json, 
                rating_json, 
                user_json 
            FROM latest_ratings
        ),
        combined_events AS (
            SELECT
                le1.user_id,
                le1.tmdb_id,
                COALESCE(le1.comment_json, le2.comment_json) AS comment_json,
                COALESCE(le1.rating_json, le2.rating_json) AS rating_json,
                le1.user_json,
                GREATEST(
                    COALESCE(le1.comment_created_at, le2.comment_created_at),
                    COALESCE(le1.rating_created_at, le2.rating_created_at)
                ) AS created_at
            FROM
                latest_events le1
            LEFT JOIN
                latest_events le2
            ON
                le1.user_id = le2.user_id
                AND le1.tmdb_id = le2.tmdb_id
                AND (
                    (le1.comment_created_at IS NOT NULL AND le2.rating_created_at IS NOT NULL AND ABS(EXTRACT(EPOCH FROM (le1.comment_created_at - le2.rating_created_at))) <= 86400)
                    OR
                    (le1.rating_created_at IS NOT NULL AND le2.comment_created_at IS NOT NULL AND ABS(EXTRACT(EPOCH FROM (le1.rating_created_at - le2.comment_created_at))) <= 86400)
                )
        ),
        distinct_events AS (
            SELECT DISTINCT ON (ce.user_id, ce.tmdb_id)
                ce.user_id,
                ce.tmdb_id,
                ce.comment_json,
                ce.rating_json,
                ce.user_json,
                ce.created_at
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
                'user', user_json,
                'created_at', created_at
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
            let events = records
                .into_iter()
                .filter_map(|value| {
                    let json = &value["event_json"];
                    match serde_json::from_value::<EventsListResult>(json.clone()) {
                        Ok(v) => Some(v),
                        Err(e) => {
                            println!("{}{:?}", LOG_KEY, e);
                            None
                        }
                    }
                })
                .collect::<Vec<EventsListResult>>();

            println!("{:?}", events);

            let mut tmdb_futures = vec![];
            let mut tmdb_movies: HashMap<i32, Option<MovieIdResult>> = HashMap::new();

            for event in events.clone() {
                let clone = http_client.clone();
                let tmdb_id = if event.comment.is_some() {
                    event.comment.unwrap().tmdb_id
                } else {
                    event.rating.unwrap().tmdb_id
                };

                let has_movie = tmdb_movies.contains_key(&tmdb_id);

                if !has_movie {
                    tmdb_movies.insert(tmdb_id, None);

                    let future = async move {
                        tmdb_movies::fetch::execute(tmdb_id as u32, Some(clone.clone())).await
                    };
                    tmdb_futures.push(future)
                } else {
                    ()
                }
            }

            tokio::join!(async {
                let mut result_vec = Vec::new();
                for future in tmdb_futures {
                    result_vec.push(future.await);
                }

                for result in result_vec {
                    match result {
                        Ok(v) => {
                            tmdb_movies.insert(v.id, Some(v));
                            ()
                        }
                        Err(e) => {
                            println!("{}{:?}", LOG_KEY, e);
                            ()
                        }
                    }
                }
            });

            Ok(events
                .into_iter()
                .map(|event| {
                    let event_clone = event.clone();

                    let tmdb_id = if event.comment.is_some() {
                        event_clone.comment.unwrap().tmdb_id
                    } else {
                        event_clone.rating.unwrap().tmdb_id
                    };

                    let movie_clone = tmdb_movies.get(&tmdb_id);

                    EventsListResult {
                        user: event.user,
                        comment: event.comment.clone(),
                        rating: event.rating.clone(),
                        tmdb_movie: if movie_clone.is_some() {
                            movie_clone.unwrap().clone()
                        } else {
                            None
                        },
                    }
                })
                .collect::<Vec<EventsListResult>>())
        }
        Err(e) => {
            println!("{}{:?}", LOG_KEY, e);
            Err(e)
        }
    }
}
