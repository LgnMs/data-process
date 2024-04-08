pub use sea_orm_migration::prelude::*;

mod m20240119_000002_create_collect_log_table;
mod m20240119_030002_create_sync_log_table;
mod m20240119_000001_create_collect_config_table;
mod m20240119_023953_create_sync_config_table;
mod m20240226_015923_create_data_source_list;
mod m20240227_022320_create_data_sharing_config_table;
mod m20240304_063328_create_sharing_request_log;
mod m20240319_061549_update_data_sharing_config_table;
mod m20240327_063820_update_sharing_request_log;
mod m20240402_033637_update_collect_config_table;
mod m20240408_033448_update_collect_log_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240119_000002_create_collect_log_table::Migration),
            Box::new(m20240119_030002_create_sync_log_table::Migration),
            Box::new(m20240119_000001_create_collect_config_table::Migration),
            Box::new(m20240119_023953_create_sync_config_table::Migration),
            Box::new(m20240226_015923_create_data_source_list::Migration),
            Box::new(m20240227_022320_create_data_sharing_config_table::Migration),
            Box::new(m20240304_063328_create_sharing_request_log::Migration),
            Box::new(m20240319_061549_update_data_sharing_config_table::Migration),
            Box::new(m20240327_063820_update_sharing_request_log::Migration),
            Box::new(m20240402_033637_update_collect_config_table::Migration),
            Box::new(m20240408_033448_update_collect_log_table::Migration),
        ]
    }
}
