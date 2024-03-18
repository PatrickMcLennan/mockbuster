use sea_orm::{DatabaseConnection, DbBackend, FromQueryResult, JsonValue, Statement};

pub struct SummaryResult {
    pub sum_score: f64,
    pub weighted_average: f64,
}

pub async fn execute(tmdb_id: i32, db: DatabaseConnection) -> Option<SummaryResult> {
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
            Some(v) => Some(SummaryResult {
                sum_score: v.get("sum_score").unwrap().as_f64().unwrap(),
                weighted_average: v.get("weighted_average").unwrap().as_f64().unwrap(),
            }),
            None => None,
        },
        Err(e) => {
            print!("Error: {:?}", e);
            None
        }
    }
}
