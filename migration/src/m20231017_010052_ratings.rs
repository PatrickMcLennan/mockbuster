use db_models::generated::{users, ratings};
use sea_orm::{
    ColumnDef, DeriveRelation, EnumIter, Related, RelationDef, RelationTrait
};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "users::Entity",
        from = "ratings::Column::UserId",
        to = "users::Column::Id"
    )]
    Users,
}

impl Related<users::Entity> for Ratings {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

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
                    .col(
                        ColumnDef::new(Ratings::UserId)
                            .integer()
                            .not_null()
                    )
                    .col(ColumnDef::new(Ratings::Score).float().not_null())
                    .col(ColumnDef::new(Ratings::MediaId).integer().not_null())
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
                    .to_owned(),
            )
            .await?;

        let conn = manager.get_connection();

        #[cfg(any(debug_assertions, test))]
        {
            conn.execute_unprepared(
                "
					INSERT INTO ratings (user_id, score, media_id) VALUES
					(1, 10, 550),
					(2, 10, 550),
					(3, 10, 550),
					(1, 10, 26679),
					(2, 10, 26679),
					(3, 10, 26679),
					(1, 7.5, 420818),
					(2, 2.0, 420818),
					(3, 6.0, 420818),
					(1, 9.0, 11362),
					(2, 7.5, 11362),
					(3, 5.0, 11362);
				",
            )
            .await?;
        }

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

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Ratings::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Ratings {
    Table,
    Id,
    UserId,
    CreatedAt,
    UpdatedAt,
    Score,
    MediaId,
}
