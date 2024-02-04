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
                    .table(SyncConfig::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SyncConfig::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(SyncConfig::DataSource)
                            .string()
                            .not_null()
                            .comment("数据源"),
                    )
                    .col(
                        ColumnDef::new(SyncConfig::SourceTableName)
                            .string()
                            .not_null()
                            .comment("数据源表名"),
                    )
                    .col(
                        ColumnDef::new(SyncConfig::QuerySql)
                            .string()
                            .not_null()
                            .comment("查询SQL"),
                    )
                    .col(
                        ColumnDef::new(SyncConfig::TargetType)
                            .string()
                            .not_null()
                            .comment("同步数据目标类型"),
                    )
                    .col(
                        ColumnDef::new(SyncConfig::TargetDataSource)
                            .string()
                            .not_null()
                            .comment("同步数据目标源"),
                    )
                    .col(
                        ColumnDef::new(SyncConfig::UpdateTime)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SyncConfig::CreateTime)
                            .timestamp()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(SyncConfig::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum SyncConfig {
    Table,
    Id,
    DataSource,
    SourceTableName,
    QuerySql,
    TargetType,
    TargetDataSource,
    UpdateTime,
    CreateTime,
}
