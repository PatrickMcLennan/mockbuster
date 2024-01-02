use sea_orm::{
    prelude::*, DatabaseBackend, DatabaseConnection, FromQueryResult, JsonValue, Statement,
};
use serde_json::{Value};

pub async fn get_recently_rented_movies(pagination: u64, db: DatabaseConnection) -> Vec<Value> {
    match JsonValue::find_by_statement(Statement::from_sql_and_values(
        DatabaseBackend::Postgres,
        r#"
				WITH latest_ratings AS (
					SELECT DISTINCT ON (r."tmdb_id")
						r."id" AS "ratings_id",
						r."user_id" AS "ratings_user_id",
						r."score",
						r."tmdb_id",
						r."created_at" AS "ratings_created_at",
						r."updated_at" AS "ratings_updated_at",
						u."id" AS "users_id",
						u."first_name",
						u."last_name",
						u."email",
						u."password_hash",
						u."permission",
						u."created_at" AS "users_created_at",
						u."updated_at" AS "users_updated_at",
						m."id" AS "movie_id",
						m."backdrop_path",
						m."title",
						m."overview",
						m."poster_path",
						m."release_date",
						m."tmdb_id" AS "movies_tmdb_id",
						m."tmdb_vote_average",
						m."tmdb_vote_count",
						m."postgres_vote_average",
						m."postgres_vote_count",
						m."created_at" AS "movies_created_at",
						m."updated_at" AS "movies_updated_at"
					FROM ratings r
					JOIN users u ON r."user_id" = u."id"
					JOIN movies m ON r."tmdb_id" = m."tmdb_id"
					ORDER BY r."tmdb_id", r."created_at" DESC
					LIMIT 20
					OFFSET $1
				)
				SELECT json_build_object(
					'movie', json_build_object(
						'id', "movie_id",
						'backdrop_path', "backdrop_path",
						'overview', "overview",
						'poster_path', "poster_path",
						'title', "title",
						'release_date', "release_date",
						'tmdb_id', "movies_tmdb_id",
						'tmdb_vote_average', "tmdb_vote_average",
						'tmdb_vote_count', "tmdb_vote_count",
						'postgres_vote_count', "postgres_vote_count",
						'postgres_vote_average', "postgres_vote_average",
						'created_at', "movies_created_at",
						'updated_at', "movies_updated_at"
					),
					'other_ratings', (
						SELECT json_agg(json_build_object(
							'rating', json_build_object(
								'created_at', r2."created_at",
								'id', r2."id",
								'score', r2."score",
								'tmdb_id', r2."tmdb_id",
								'updated_at', r2."updated_at",
								'user_id', r2."user_id"
							),
							'user', json_build_object(
								'id', u2."id",
								'first_name', u2."first_name",
								'last_name', u2."last_name",
								'password_hash', u2."password_hash",
								'email', u2."email",
								'permission', u2."permission",
								'created_at', u2."created_at",
								'updated_at', u2."updated_at"
							)
						))
						FROM ratings r2
						JOIN users u2 ON r2."user_id" = u2."id"
						WHERE r2."tmdb_id" = latest_ratings."tmdb_id"
						AND r2."user_id" != latest_ratings."ratings_user_id"
					),
					'rating', json_build_object(
						'created_at', latest_ratings."ratings_created_at",
						'id', latest_ratings."ratings_id",
						'score', latest_ratings."score",
						'tmdb_id', latest_ratings."tmdb_id",
						'updated_at', latest_ratings."ratings_updated_at",
						'user_id', latest_ratings."ratings_user_id"
					),
					'user', json_build_object(
						'created_at', latest_ratings."users_created_at",
						'updated_at', latest_ratings."users_updated_at",
						'first_name', latest_ratings."first_name",
						'last_name', latest_ratings."last_name",
						'email', latest_ratings."email",
						'password_hash', latest_ratings."password_hash",
						'permission', latest_ratings."permission",
						'id', latest_ratings."users_id"
					)
				) AS rating_result
				FROM latest_ratings;						  
			"#,
        [((pagination - 1) * 20).into()],
    ))
    .all(&db)
    .await
    {
        Ok(results) => results,
        Err(e) => {
            println!("Error: {:?}", e);
            vec![]
        }
    }
}
