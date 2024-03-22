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
                    .table(Comments::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Comments::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Comments::UserId).integer().not_null())
                    .col(ColumnDef::new(Comments::Content).string().not_null())
                    .col(
                        ColumnDef::new(Comments::CreatedAt)
                            .timestamp_with_time_zone()
                            .default("now()")
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Comments::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default("now()")
                            .not_null(),
                    )
                    .col(ColumnDef::new(Comments::TmdbId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("comments-user-id")
                            .from(Comments::Table, Comments::UserId)
                            .to(Users::Table, Users::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("comments-created_at-index")
                    .table(Comments::Table)
                    .col(Comments::CreatedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("comments-tmdb_id-index")
                    .table(Comments::Table)
                    .col(Comments::TmdbId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("comments-user_id-index")
                    .table(Comments::Table)
                    .col(Comments::UserId)
                    .to_owned(),
            )
            .await?;

        let conn = manager.get_connection();

        conn.execute_unprepared(
            "
                    CREATE TRIGGER trigger_update_comments_updated_at
                    BEFORE UPDATE
                    ON comments
                    FOR EACH ROW
                    EXECUTE FUNCTION update_updated_at_column();
                ",
        )
        .await?;

        if cfg!(any(debug_assertions, test)) {
            conn.execute_unprepared(
                "
                        INSERT INTO comments (user_id, tmdb_id, content) VALUES
                        (1, 550, 'This is a good movie.'),
                        (1, 26679, 'This is a bad movie.'),
                        (1, 11326, 'Did not like.'),
                        (1, 420818, 'Could have been better.'),
                        (2, 550, 'This is a bad movie.'),
                        (2, 26679, 'I disagree, this is not good.'),
                        (2, 11326, 'One of my favourites!'),
                        (2, 420818, 'Boooo'),
                        (3, 550, 'Great!'),
                        (3, 26679, 'Awful!'),
                        (3, 11326, 'Pass on this one'),
                        (3, 420818, 'Bad');
                    ",
            )
            .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Comments::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Comments {
    Table,
    Id,
    Content,
    CreatedAt,
    TmdbId,
    UserId,
    UpdatedAt,
}
