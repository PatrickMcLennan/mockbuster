use sea_orm::{
    DeriveActiveEnum, DeriveRelation, EnumIter, Related, RelationDef, RelationTrait,
};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "db_models::generated::ratings::Entity")]
    Ratings,
}

impl Related<db_models::generated::ratings::Entity> for Users {
    fn to() -> RelationDef {
        Relation::Ratings.def()
    }
}

#[derive(EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum Permission {
    User = 0,
    Admin = 1,
    Owner = 2,
}

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
                    .col(
                        ColumnDef::new(Users::CreatedAt)
                            .timestamp_with_time_zone()
                            .default("now()")
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Users::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default("now()")
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        let conn = manager.get_connection();

        if cfg!(any(debug_assertions, test)) {
            conn.execute_unprepared(
                "
					INSERT INTO users (first_name, last_name, email, password_hash, permission) VALUES
					('Elvis', 'Presley', 'king@theking.com', crypt('!Testing2', gen_salt('bf')), 2),
					('Kurt', 'Cobain', 'whatever@whatever.com', crypt('!Testing0', gen_salt('bf')), 0),
					('Jimi', 'Hendrix', 'jimi@hendrix.com', crypt('!Testing0', gen_salt('bf')), 0);
				",
            )
            .await?;
        }

        conn.execute_unprepared(
            "
				CREATE TRIGGER trigger_update_users_updated_at
				BEFORE UPDATE
				ON users
				FOR EACH ROW
				EXECUTE FUNCTION update_updated_at_column();
			",
        )
        .await?;

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
    CreatedAt,
    UpdatedAt,
}
