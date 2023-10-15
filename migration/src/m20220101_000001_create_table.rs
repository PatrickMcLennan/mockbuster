use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Users::FirstName).string().not_null())
                    .col(ColumnDef::new(Users::LastName).string().not_null())
                    .col(ColumnDef::new(Users::PasswordHash).string().not_null())
                    .col(ColumnDef::new(Users::Email).string().not_null())
                    .col(ColumnDef::new(Users::Permission).integer().not_null())
                    .to_owned(),
            )
            .await?;

        if cfg!(any(debug_assertions, test)) {
            let conn = manager.get_connection();

            conn.execute_unprepared(
                "INSERT INTO users (first_name, last_name, email, password_hash, permission) VALUES
					('Elvis', 'Presley', 'king@theking.com', crypt('!Testing2', gen_salt('md5')), 2),
					('Kurt', 'Cobain', 'whatever@whatever.com', crypt('!Testing0', gen_salt('md5')), 0),
					('Jimi', 'Hendrix', 'jimi@hendrix.com', crypt('!Testing0', gen_salt('md5')), 0);",
            )
            .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Users {
    Table,
    Id,
    FirstName,
    LastName,
    Email,
    PasswordHash,
    Permission,
}
