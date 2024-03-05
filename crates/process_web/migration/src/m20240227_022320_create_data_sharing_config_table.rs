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
                    .table(DataSharingConfig::Table)
                    .comment("共享配置")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DataSharingConfig::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(DataSharingConfig::Name).string().not_null())
                    .col(
                        ColumnDef::new(DataSharingConfig::TableName)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(DataSharingConfig::QuerySql)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(DataSharingConfig::DataSourceId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(DataSharingConfig::DelFlag)
                            .integer()
                            .default(0)
                            .not_null()
                            .comment(r#"1 已删除 0 未删除"#),
                    )
                    .col(
                        ColumnDef::new(DataSharingConfig::UpdateTime)
                            .date_time()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(DataSharingConfig::CreateTime)
                            .date_time()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(DataSharingConfig::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum DataSharingConfig {
    Table,
    Id,
    Name,
    TableName,
    QuerySql,
    DataSourceId,
    DelFlag,
    UpdateTime,
    CreateTime,
}
