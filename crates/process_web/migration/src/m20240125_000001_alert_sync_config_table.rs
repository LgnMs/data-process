use crate::m20240119_023953_create_sync_config_table::SyncConfig;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .alter_table(
                Table::alter()
                    .table(SyncConfig::Table)
                    .add_column(
                        ColumnDef::new(SyncConfig::UpdateTime)
                            .timestamp()
                            .not_null(),
                    )
                    .add_column(
                        ColumnDef::new(SyncConfig::CreateTime)
                            .timestamp()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }
}
