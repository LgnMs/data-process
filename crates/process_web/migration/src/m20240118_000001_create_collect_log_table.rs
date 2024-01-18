use crate::m20240118_000001_create_collect_config_table::CollectConfig;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .comment("数据采集日志")
                    .table(CollectLog::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CollectLog::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(CollectLog::RunningLog).string().not_null())
                    .col(
                        ColumnDef::new(CollectLog::CollectConfigId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("collect_log_collect_config_fk")
                            .from(CollectLog::Table, CollectLog::CollectConfigId)
                            .to(CollectConfig::Table, CollectConfig::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(CollectConfig::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum CollectLog {
    Table,
    Id,
    RunningLog,
    CollectConfigId,
}
