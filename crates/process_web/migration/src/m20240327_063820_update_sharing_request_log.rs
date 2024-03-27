use sea_orm_migration::prelude::*;

use crate::m20240304_063328_create_sharing_request_log::SharingRequestLog;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
        .alter_table(
            Table::alter()
                .table(SharingRequestLog::Table)
                .add_column(
                    ColumnDef::new(SharingRequestLog::UserInfo)
                        .string()
                        .comment("用户信息"),
                )
                .to_owned(),
        )
        .await
    }

}

