use sea_orm_migration::prelude::*;

use crate::m20240119_000002_create_collect_log_table::CollectLog;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(CollectLog::Table)
                    .add_column(
                        ColumnDef::new(CollectLog::TaskId)
                            .string()
                            .comment(r#"正在执行的任务的id"#),
                    )
                    .to_owned()
            )
            .await
    }

}
