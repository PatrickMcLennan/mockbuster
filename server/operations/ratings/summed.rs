use sea_orm::{DatabaseConnection, DbBackend, DbErr, FromQueryResult, JsonValue, Statement};

pub struct SummaryResult {
    pub sum_score: f64,
    pub weighted_average: f64,
}

const LOG_KEY: &str = "[Operations::Ratings::Summed]: ";

pub async fn execute(tmdb_id: i32, db: DatabaseConnection) -> Result<SummaryResult, DbErr> {
    match JsonValue::find_by_statement(Statement::from_sql_and_values(
        DbBackend::Postgres,
        r#"
            SELECT 
                tmdb_id, 
                SUM(score) AS sum_score,
                ROUND(SUM(score) / COUNT(*) * (1 - EXP(-2))) / 2 * 2.0 AS weighted_average
            FROM 
                ratings
            WHERE 
                tmdb_id = $1
            GROUP BY 
                tmdb_id;
            "#,
        [tmdb_id.into()],
    ))
    .one(&db)
    .await
    {
        Ok(v) => match v {
            Some(v) => Ok(SummaryResult {
                sum_score: v.get("sum_score").unwrap().as_f64().unwrap(),
                weighted_average: v.get("weighted_average").unwrap().as_f64().unwrap(),
            }),
            None => {
                println!("{}: Cannot sum ratings for {}", LOG_KEY, tmdb_id);
                Err(DbErr::RecordNotFound(tmdb_id.to_string()))
            }
        },
        Err(e) => {
            print!("{}{:?}", LOG_KEY, e);
            Err(e)
        }
    }
}
