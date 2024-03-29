use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(DataSourceList::Table)
                    .comment("数据源列表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DataSourceList::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(DataSourceList::DatabaseName)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(DataSourceList::Name)
                            .comment(r#"数据源名称"#)
                            .string(),
                    )
                    .col(ColumnDef::new(DataSourceList::TableSchema).string())
                    .col(
                        ColumnDef::new(DataSourceList::DatabaseType)
                            .string()
                            .comment(r#"数据库类型"#)
                            .not_null(),
                    )
                    .col(ColumnDef::new(DataSourceList::Host).string().not_null())
                    .col(ColumnDef::new(DataSourceList::Port).string().not_null())
                    .col(ColumnDef::new(DataSourceList::User).string().not_null())
                    .col(ColumnDef::new(DataSourceList::Password).string().not_null())
                    .col(
                        ColumnDef::new(DataSourceList::DelFlag)
                            .integer()
                            .default(0)
                            .not_null()
                            .comment(r#"1 已删除 0 未删除"#),
                    )
                    .col(
                        ColumnDef::new(DataSourceList::UpdateTime)
                            .date_time()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(DataSourceList::CreateTime)
                            .date_time()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(DataSourceList::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum DataSourceList {
    Table,
    Id,
    DatabaseName,
    Name,
    TableSchema,
    DatabaseType,
    Host,
    Port,
    User,
    Password,
    DelFlag,
    UpdateTime,
    CreateTime,
}
