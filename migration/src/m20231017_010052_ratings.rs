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

        conn.execute_unprepared(
            "
				CREATE TRIGGER trigger_update_users_updated_at
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
	
}
