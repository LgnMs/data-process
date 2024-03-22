use crate::m20240227_022320_create_data_sharing_config_table::DataSharingConfig;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(DataSharingConfig::Table)
                    .add_column(
                        ColumnDef::new(DataSharingConfig::ApiId)
                            .string()
                            .comment("api调用id"),
                    )
                    .to_owned(),
            )
            .await
    }
}
