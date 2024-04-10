use sea_orm_migration::prelude::*;

use crate::m20240119_000001_create_collect_config_table::CollectConfig;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(CollectConfig::Table)
                    .add_column(
                        ColumnDef::new(CollectConfig::DbColumnsConfig2)
                            .json()
                            .comment(r#"数据库列配置2"#),
                    )
                    .to_owned(),
            )
            .await
    }
}
