use super::m20231017_010052_ratings::Ratings;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AggregateRatings::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AggregateRatings::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(AggregateRatings::Score).float().not_null())
                    .col(
                        ColumnDef::new(Ratings::TmdbId)
                            .integer()
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AggregateRatings::CreatedAt)
                            .timestamp_with_time_zone()
                            .default("now()")
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AggregateRatings::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default("now()")
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("aggregate_ratings-rating-id")
                            .from(Ratings::TmdbId, Ratings::TmdbId)
                            .to(AggregateRatings::Table, AggregateRatings::TmdbId),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("aggregate_ratings-tmdb_id-index")
                    .table(AggregateRatings::Table)
                    .col(AggregateRatings::TmdbId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        let conn = manager.get_connection();

        conn.execute_unprepared(
            "
                    CREATE TRIGGER trigger_update_aggregate_ratings_updated_at
                    BEFORE UPDATE
                    ON aggregate_ratings
                    FOR EACH ROW
                    EXECUTE FUNCTION update_updated_at_column();
                ",
        )
        .await?;

        if cfg!(any(debug_assertions, test)) {
            conn.execute_unprepared(
                "
                    WITH RatingCounts AS (
                        SELECT tmdb_id, COUNT(*) AS count_per_tmdb_id
                        FROM ratings
                        GROUP BY tmdb_id
                    )
                    INSERT INTO aggregate_ratings (score, tmdb_id)
                    SELECT ROUND((SUM(r.score) / rc.count_per_tmdb_id) * (1 - EXP(-2))) / 2 * 2.0 AS rounded_score, r.tmdb_id
                    FROM ratings AS r
                    JOIN RatingCounts AS rc ON r.tmdb_id = rc.tmdb_id
                    GROUP BY r.tmdb_id, rc.count_per_tmdb_id;                
                ",
            )
            .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        if manager.has_table("aggregate_ratings").await? {
            manager
                .drop_table(Table::drop().table(AggregateRatings::Table).to_owned())
                .await
        } else {
            Ok(())
        }
    }
}

#[derive(DeriveIden)]
enum AggregateRatings {
    Table,
    Id,
    Score,
    CreatedAt,
    UpdatedAt,
    TmdbId,
}
