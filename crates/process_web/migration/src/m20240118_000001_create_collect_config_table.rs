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
                            .string()
                            .comment("请求头"),
                    )
                    .col(
                        ColumnDef::new(CollectConfig::Body)
                            .string()
                            .comment("请求体"),
                    )
                    .col(
                        ColumnDef::new(CollectConfig::MapRules)
                            .string()
                            .comment(r#"数据映射关系: [["code", "code2"]]"#),
                    )
                    .col(
                        ColumnDef::new(CollectConfig::TemplateString)
                            .string()
                            .not_null()
                            .comment("导出字符串模板"),
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
    Url,
    Method,
    Headers,
    Body,
    MapRules,
    TemplateString,
}
