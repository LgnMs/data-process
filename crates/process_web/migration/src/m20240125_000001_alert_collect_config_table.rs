use crate::m20240119_000001_create_collect_config_table::CollectConfig;
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
                    .table(CollectConfig::Table)
                    .add_column(
                        ColumnDef::new(CollectConfig::UpdateTime)
                            .timestamp()
                            .not_null(),
                    )
                    .add_column(
                        ColumnDef::new(CollectConfig::CreateTime)
                            .timestamp()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }
}
