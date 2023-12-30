pub use sea_orm_migration::prelude::*;

mod m20231229_022203_user_table;
mod m20231229_232418_url_data;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20231229_022203_user_table::Migration),
            Box::new(m20231229_232418_url_data::Migration),
        ]
    }
}
