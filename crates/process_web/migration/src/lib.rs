pub use sea_orm_migration::prelude::*;

mod m20240118_000001_create_collect_log_table;
mod m20240118_000001_create_sync_log_table;
mod m20240119_000001_create_collect_config_table;
mod m20240119_023953_create_sync_config_table;
mod m20240226_015923_create_data_source_list;
mod m20240227_022320_create_data_sharing_config_table;
mod m20240304_063328_create_sharing_request_log;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240118_000001_create_collect_log_table::Migration),
            Box::new(m20240118_000001_create_sync_log_table::Migration),
            Box::new(m20240119_000001_create_collect_config_table::Migration),
            Box::new(m20240119_023953_create_sync_config_table::Migration),
            Box::new(m20240226_015923_create_data_source_list::Migration),
            Box::new(m20240227_022320_create_data_sharing_config_table::Migration),
            Box::new(m20240304_063328_create_sharing_request_log::Migration),
        ]
    }
}
