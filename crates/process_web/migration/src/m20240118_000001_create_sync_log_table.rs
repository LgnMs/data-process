use crate::m20240119_023953_create_sync_config_table::SyncConfig;
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
                    .comment("数据同步日志")
                    .table(SyncLog::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(SyncLog::Id).uuid().not_null().primary_key())
                    .col(
                        ColumnDef::new(SyncLog::RunningLog)
                            .text()
                            .not_null()
                            .comment("采集日志"),
                    )
                    .col(
                        ColumnDef::new(SyncLog::Status)
                            .integer()
                            .not_null()
                            .default(0)
                            .comment("0 未开始 1 运行中 2 成功 3 失败 4 等待线程分配"),
                    )
                    .col(
                        ColumnDef::new(SyncLog::SyncConfigId)
                            .integer()
                            .not_null()
                            .comment("采集配置项FK—ID"),
                    )
                    .col(ColumnDef::new(SyncLog::UpdateTime).date_time().not_null())
                    .col(ColumnDef::new(SyncLog::CreateTime).date_time().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("sync_log_sync_config_fk")
                            .from(SyncLog::Table, SyncLog::SyncConfigId)
                            .to(SyncConfig::Table, SyncConfig::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(SyncLog::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum SyncLog {
    Table,
    Id,
    RunningLog,
    SyncConfigId,
    Status,
    UpdateTime,
    CreateTime,
}
