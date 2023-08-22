pub use sea_orm_migration::prelude::*;

mod m20230821_102932_create_user;
mod m20230822_023238_create_block;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230821_102932_create_user::Migration),
            Box::new(m20230822_023238_create_block::Migration),
        ]
    }
}
