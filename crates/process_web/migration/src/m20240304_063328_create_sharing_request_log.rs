use crate::m20240227_022320_create_data_sharing_config_table::DataSharingConfig;
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
                    .comment("数据共享请求信息记录日志")
                    .table(SharingRequestLog::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SharingRequestLog::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(SharingRequestLog::Log)
                            .text()
                            .not_null()
                            .comment("日志"),
                    )
                    .col(
                        ColumnDef::new(SharingRequestLog::DataSharingConfigId)
                            .integer()
                            .not_null()
                            .comment("共享配置FK—ID"),
                    )
                    .col(
                        ColumnDef::new(SharingRequestLog::UpdateTime)
                            .date_time()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SharingRequestLog::CreateTime)
                            .date_time()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("data_sharing_config_fk")
                            .from(
                                SharingRequestLog::Table,
                                SharingRequestLog::DataSharingConfigId,
                            )
                            .to(DataSharingConfig::Table, DataSharingConfig::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(SharingRequestLog::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum SharingRequestLog {
    Table,
    Id,
    Log,
    UserInfo,
    DataSharingConfigId,
    UpdateTime,
    CreateTime,
}
