//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.10

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, TS, Default)]
#[sea_orm(table_name = "collect_config")]
#[ts(
    export,
    export_to = "ui/api/models/auto-generates/CollectConfig.ts",
    rename = "CollectConfig"
)]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub url: String,
    pub name: String,
    pub desc: Option<String>,
    pub method: String,
    #[ts(type = "any")]
    pub headers: Option<Json>,
    pub body: Option<String>,
    #[ts(type = "any")]
    pub map_rules: Option<Json>,
    pub template_string: String,
    pub loop_request_by_pagination: Option<bool>,
    pub max_number_of_result_data: Option<i32>,
    pub filed_of_result_data: Option<String>,
    pub max_count_of_request: Option<i32>,
    pub cache_table_name: Option<String>,
    #[ts(type = "any")]
    pub nested_config: Option<Json>,
    #[ts(type = "any")]
    pub db_columns_config: Option<Json>,
    #[ts(type = "any")]
    pub db_columns_config2: Option<Json>,
    pub cron: Option<String>,
    pub job_id: Option<Uuid>,
    #[serde(skip_deserializing)]
    pub update_time: DateTime,
    #[serde(skip_deserializing)]
    pub create_time: DateTime,
    #[serde(skip_deserializing)]
    pub del_flag: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::collect_log::Entity")]
    CollectLog,
}

impl Related<super::collect_log::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CollectLog.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
