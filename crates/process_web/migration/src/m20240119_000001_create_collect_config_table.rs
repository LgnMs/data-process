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
                    .table(CollectConfig::Table)
                    .comment("数据采集配置")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CollectConfig::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(CollectConfig::Name)
                            .string()
                            .not_null()
                            .comment("名称"),
                    )
                    .col(ColumnDef::new(CollectConfig::Desc).string().comment("描述"))
                    .col(
                        ColumnDef::new(CollectConfig::Url)
                            .string()
                            .not_null()
                            .comment("api地址"),
                    )
                    .col(
                        ColumnDef::new(CollectConfig::Method)
                            .string()
                            .not_null()
                            .comment("请求类型: GET, POST, ..."),
                    )
                    .col(
                        ColumnDef::new(CollectConfig::Headers)
                            .json()
                            .comment("请求头"),
                    )
                    .col(
                        ColumnDef::new(CollectConfig::Body)
                            .string()
                            .comment("请求体"),
                    )
                    .col(
                        ColumnDef::new(CollectConfig::MapRules)
                            .json()
                            .comment(r#"数据映射关系: [["code", "code2"]]"#),
                    )
                    .col(
                        ColumnDef::new(CollectConfig::TemplateString)
                            .string()
                            .not_null()
                            .comment("导出字符串模板"),
                    )
                    .col(
                        ColumnDef::new(CollectConfig::LoopRequestByPagination)
                            .boolean()
                            .default(false)
                            .comment("是否通过分页值循环请求，直到数据请求完毕"),
                    )
                    .col(
                        ColumnDef::new(CollectConfig::CacheTableName)
                            .string()
                            .default(false)
                            .comment("暂存数据库表名，存储接收并处理后的数据"),
                    )
                    .col(
                        ColumnDef::new(CollectConfig::Cron)
                            .string()
                            .comment("任务调度时间 Cron表达式"),
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
pub enum CollectConfig {
    Table,
    Id,
    Name,
    Desc,
    Url,
    Method,
    Headers,
    Body,
    MapRules,
    TemplateString,
    LoopRequestByPagination,
    CacheTableName,
    MaxNumberOfResultData,
    FiledOfResultData,
    MaxCountOfRequest,
    DbColumnsConfig,
    Cron,
    UpdateTime,
    CreateTime,
    DelFlag,
}
