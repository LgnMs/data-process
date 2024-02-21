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
                    .col(ColumnDef::new(SyncConfig::Name).string().not_null())
                    .col(
                        ColumnDef::new(SyncConfig::DataSource)
                            .json()
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
                        ColumnDef::new(SyncConfig::SourceTableColumns)
                            .json()
                            .not_null()
                            .comment("数据源要查询的字段"),
                    )
                    .col(
                        ColumnDef::new(SyncConfig::QuerySql)
                            .string()
                            .not_null()
                            .comment("数据源要执行的查询SQL"),
                    )
                    .col(
                        ColumnDef::new(SyncConfig::TargetDataSource)
                            .json()
                            .not_null()
                            .comment("同步数据目标源"),
                    )
                    .col(
                        ColumnDef::new(SyncConfig::TargetTableName)
                            .string()
                            .not_null()
                            .comment("同步数据目标表"),
                    )
                    .col(
                        ColumnDef::new(SyncConfig::TargetQuerySqlTemplate)
                            .string()
                            .not_null()
                            .comment("目标数据库要执行的sql模板"),
                    )
                    .col(
                        ColumnDef::new(SyncConfig::JobId)
                            .uuid()
                            .comment(r#"调度任务ID"#),
                    )
                    .col(
                        ColumnDef::new(SyncConfig::Cron)
                            .string()
                            .comment("任务调度时间 Cron表达式"),
                    )
                    .col(
                        ColumnDef::new(SyncConfig::DelFlag)
                            .integer()
                            .default(0)
                            .comment(r#"1 已删除 0 未删除"#),
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
    Name,
    DataSource,
    SourceTableName,
    SourceTableColumns,
    QuerySql,
    TargetDataSource,
    TargetTableName,
    TargetQuerySqlTemplate,
    Cron,
    JobId,
    DelFlag,
    UpdateTime,
    CreateTime,
}
