pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20231017_010052_ratings;
mod m20240317_172504_aggregate_ratings;
mod m20240321_234206_create_comments;
mod m20240323_174030_create_notifications;
mod m20240323_181246_create_user_preferences;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20231017_010052_ratings::Migration),
            Box::new(m20240317_172504_aggregate_ratings::Migration),
            Box::new(m20240321_234206_create_comments::Migration),
            Box::new(m20240323_174030_create_notifications::Migration),
            Box::new(m20240323_181246_create_user_preferences::Migration),
        ]
    }
}
