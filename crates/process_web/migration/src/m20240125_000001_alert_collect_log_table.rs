use sea_orm_migration::prelude::*;
use crate::m20240118_000001_create_collect_log_table::CollectLog;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .alter_table(
                Table::alter()
                    .table(CollectLog::Table)
                    .add_column(
                            ColumnDef::new(CollectLog::UpdateTime)
                                .timestamp()
                                .not_null(),
                    )
                    .add_column(
                        ColumnDef::new(CollectLog::CreateTime)
                            .timestamp()
                            .not_null(),
                    )
                    .to_owned()
            )
            .await
    }
}

