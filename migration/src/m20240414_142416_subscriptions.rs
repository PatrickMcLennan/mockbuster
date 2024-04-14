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
                    .table(Subscriptions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Subscriptions::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Subscriptions::Endpoint).string().not_null())
                    .col(ColumnDef::new(Subscriptions::P256).string().not_null())
                    .col(ColumnDef::new(Subscriptions::Auth).string().not_null())
                    .col(ColumnDef::new(Subscriptions::UserId).integer().not_null())
                    .col(
                        ColumnDef::new(Subscriptions::CreatedAt)
                            .timestamp_with_time_zone()
                            .default("now()")
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Subscriptions::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default("now()")
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("subscriptions-user-id")
                            .from(Subscriptions::Table, Subscriptions::UserId)
                            .to(Users::Table, Users::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("subscriptions-created_at-index")
                    .table(Subscriptions::Table)
                    .col(Subscriptions::CreatedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("subscriptions-user_id-index")
                    .table(Subscriptions::Table)
                    .col(Subscriptions::UserId)
                    .to_owned(),
            )
            .await?;

        let conn = manager.get_connection();

        conn.execute_unprepared(
            "
                        CREATE TRIGGER trigger_update_subscriptions_updated_at
                        BEFORE UPDATE
                        ON subscriptions
                        FOR EACH ROW
                        EXECUTE FUNCTION update_updated_at_column();
                    ",
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Subscriptions::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Subscriptions {
    Table,
    Id,
    Endpoint,
    P256,
    Auth,
    UserId,
    CreatedAt,
    UpdatedAt,
}
