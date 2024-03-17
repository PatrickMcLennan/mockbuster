use super::m20220101_000001_create_table::Users;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Ratings::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Ratings::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Ratings::UserId).integer().not_null())
                    .col(ColumnDef::new(Ratings::Score).float().not_null())
                    .col(ColumnDef::new(Ratings::TmdbId).integer().not_null())
                    .col(
                        ColumnDef::new(Ratings::CreatedAt)
                            .timestamp_with_time_zone()
                            .default("now()")
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Ratings::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default("now()")
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("ratings-user-id")
                            .from(Ratings::Table, Ratings::UserId)
                            .to(Users::Table, Users::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("ratings-created_at-index")
                    .table(Ratings::Table)
                    .col(Ratings::CreatedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("ratings-tmdb_id-index")
                    .table(Ratings::Table)
                    .col(Ratings::TmdbId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("ratings-user_id-index")
                    .table(Ratings::Table)
                    .col(Ratings::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("ratings-user_id-tmdb_id-index")
                    .table(Ratings::Table)
                    .col(Ratings::UserId)
                    .col(Ratings::TmdbId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        let conn = manager.get_connection();

        conn.execute_unprepared(
            "
				CREATE TRIGGER trigger_update_ratings_updated_at
				BEFORE UPDATE
				ON ratings
				FOR EACH ROW
				EXECUTE FUNCTION update_updated_at_column();
			",
        )
        .await?;

        conn.execute_unprepared(
            "
                    INSERT INTO ratings (user_id, score, tmdb_id) VALUES
                    (1, 10, 550),
                    (2, 10, 550),
                    (3, 10, 550),
                    (1, 10, 26679),
                    (2, 10, 26679),
                    (3, 10, 26679),
                    (1, 9.0, 11362),
                    (2, 7.5, 11362),
                    (3, 5.0, 11362),
                    (1, 7.5, 420818),
                    (2, 2.0, 420818),
                    (3, 6.0, 420818);
                ",
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        if manager.has_table("ratings").await? {
            manager
                .drop_table(Table::drop().table(Ratings::Table).to_owned())
                .await
        } else {
            Ok(())
        }
    }
}

#[derive(DeriveIden)]
pub enum Ratings {
    Table,
    Id,
    UserId,
    CreatedAt,
    UpdatedAt,
    Score,
    TmdbId,
}
