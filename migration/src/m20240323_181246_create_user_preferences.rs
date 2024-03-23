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
                    .table(UserPreference::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserPreference::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UserPreference::UserId).integer().not_null())
                    .col(
                        ColumnDef::new(UserPreference::AllowComments)
                            .boolean()
                            .default(true)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserPreference::AllowRatings)
                            .boolean()
                            .default(true)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserPreference::IgnoredUserRatings)
                            .array(ColumnType::BigInteger),
                    )
                    .col(
                        ColumnDef::new(UserPreference::IgnoredUserComments)
                            .array(ColumnType::BigInteger),
                    )
                    .col(
                        ColumnDef::new(UserPreference::CreatedAt)
                            .timestamp_with_time_zone()
                            .default("now()")
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserPreference::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default("now()")
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("user-preferences-user-id")
                            .from(UserPreference::Table, UserPreference::UserId)
                            .to(Users::Table, Users::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("user-preferences-created_at-index")
                    .table(UserPreference::Table)
                    .col(UserPreference::CreatedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("user-preferences-user_id-index")
                    .table(UserPreference::Table)
                    .col(UserPreference::UserId)
                    .to_owned(),
            )
            .await?;

        let conn = manager.get_connection();

        conn.execute_unprepared(
            "
                    CREATE TRIGGER trigger_update_comments_updated_at
                    BEFORE UPDATE
                    ON user_preference
                    FOR EACH ROW
                    EXECUTE FUNCTION update_updated_at_column();
                ",
        )
        .await?;

        if cfg!(any(debug_assertions, test)) {
            conn.execute_unprepared(
                "
                        INSERT INTO user_preference (user_id, allow_comments, allow_ratings) VALUES
                        (1, true, true),
                        (2, true, true),
                        (3, true, true);
                    ",
            )
            .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserPreference::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UserPreference {
    Table,
    Id,
    UserId,
    AllowComments,
    AllowRatings,
    IgnoredUserRatings,
    IgnoredUserComments,
    CreatedAt,
    UpdatedAt,
}
