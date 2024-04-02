use crate::m20240119_000001_create_collect_config_table::CollectConfig;
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
                    .col(
                        ColumnDef::new(CollectLog::RunningLog)
                            .text()
                            .not_null()
                            .comment("采集日志"),
                    )
                    .col(
                        ColumnDef::new(CollectLog::Status)
                            .integer()
                            .not_null()
                            .default(0)
                            .comment("0 未开始 1 运行中 2 成功 3 失败 4 等待线程分配"),
                    )
                    .col(
                        ColumnDef::new(CollectLog::CollectConfigId)
                            .integer()
                            .not_null()
                            .comment("采集配置项FK—ID"),
                    )
                    .col(
                        ColumnDef::new(CollectLog::UpdateTime)
                            .date_time()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CollectLog::CreateTime)
                            .date_time()
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
            .drop_table(Table::drop().table(CollectLog::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum CollectLog {
    Table,
    Id,
    RunningLog,
    CollectConfigId,
    Status,
    UpdateTime,
    CreateTime,
}
