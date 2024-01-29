use crate::m20240119_000001_create_collect_config_table::CollectConfig;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .alter_table(
                Table::alter()
                    .table(CollectConfig::Table)
                    .add_column(
                        ColumnDef::new(CollectConfig::MaxNumberOfResultData)
                            .integer()
                            .comment("返回数据的最大数量限制，一旦已保存的数据超过该值便不会再发起请求")
                            .default(1000)
                    )
                    .add_column(
                        ColumnDef::new(CollectConfig::FiledOfResultData)
                            .string()
                            .comment(r#"返回数据中应检测的list的字段名，例如{"result": "data":[]}"#)
                    )
                    .add_column(
                        ColumnDef::new(CollectConfig::MaxCountOfRequest)
                            .string()
                            .comment(r#"最大请求次数"#)
                    )
                    .to_owned(),
            )
            .await
    }
}
