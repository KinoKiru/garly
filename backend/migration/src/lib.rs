pub use sea_orm_migration::prelude::*;
mod extension;

mod m20230506_140209_update_timestamp_trigger;
mod m20230506_153115_user_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230506_140209_update_timestamp_trigger::Migration),
            Box::new(m20230506_153115_user_table::Migration),
        ]
    }
}
