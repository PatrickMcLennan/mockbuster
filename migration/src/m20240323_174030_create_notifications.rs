use super::m20220101_000001_create_table::Users;
use sea_orm::{DeriveActiveEnum, EnumIter};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum NotificationType {
    Rating = 0,
    Comment = 1,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Notifications::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Notifications::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Notifications::UserId).integer().not_null())
                    .col(
                        ColumnDef::new(Notifications::NotificationType)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Notifications::Seen)
                            .boolean()
                            .default(false)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Notifications::RelatedId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Notifications::CreatedAt)
                            .timestamp_with_time_zone()
                            .default("now()")
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Notifications::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default("now()")
                            .not_null(),
                    )
                    .col(ColumnDef::new(Notifications::SeenAt).timestamp_with_time_zone())
                    .foreign_key(
                        ForeignKey::create()
                            .name("notifications-user-id")
                            .from(Notifications::Table, Notifications::UserId)
                            .to(Users::Table, Users::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("notifications-created_at-index")
                    .table(Notifications::Table)
                    .col(Notifications::CreatedAt)
                    .to_owned(),
            )
            .await?;
    
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("notifications-user_id-index")
                    .table(Notifications::Table)
                    .col(Notifications::UserId)
                    .to_owned(),
            )
            .await?;

        let conn = manager.get_connection();

        // if cfg!(any(debug_assertions, test)) {
        //     conn.execute_unprepared(
        //             "
        //                 INSERT INTO notifications (first_name, last_name, email, password_hash, permission) VALUES
        //                 ('Elvis', 'Presley', 'king@theking.com', crypt('!Testing2', gen_salt('bf')), 2),
        //                 ('Kurt', 'Cobain', 'whatever@whatever.com', crypt('!Testing0', gen_salt('bf')), 0),
        //                 ('Jimi', 'Hendrix', 'jimi@hendrix.com', crypt('!Testing0', gen_salt('bf')), 0);
        //             ",
        //         )
        //         .await?;
        // }

        conn.execute_unprepared(
            "
                    CREATE TRIGGER trigger_update_notifications_updated_at
                    BEFORE UPDATE
                    ON notifications
                    FOR EACH ROW
                    EXECUTE FUNCTION update_updated_at_column();
                ",
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Notifications::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Notifications {
    Table,
    Id,
    NotificationType,
    UserId,
    RelatedId,
    Seen,
    CreatedAt,
    SeenAt,
    UpdatedAt,
}
