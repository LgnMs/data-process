use sea_orm::ActiveValue::{Set, Unchanged};
use sea_orm::*;
use tracing::debug;
use process_core::db::DataSource;

use crate::api::data_source_list::ListParams;
use crate::entity::data_source_list;
use crate::entity::data_source_list::Model;

pub struct DataSourceListService;

impl DataSourceListService {
    pub async fn find_by_id(db: &DbConn, id: i32) -> Result<Model, DbErr> {
        data_source_list::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find data by id.".to_owned()))
    }

    pub async fn list(
        db: &DbConn,
        page: u64,
        page_size: u64,
        data: Option<ListParams>,
    ) -> Result<(Vec<Model>, u64), DbErr> {
        let mut conditions = Condition::all();
        if let Some(data) = data {
            if let Some(name) = data.database_name {
                conditions = conditions.add(data_source_list::Column::DatabaseName.contains(&name));
            }
        }

        let paginator = data_source_list::Entity::find()
            .filter(data_source_list::Column::DelFlag.eq(0))
            .filter(conditions)
            .order_by_desc(data_source_list::Column::Id)
            .paginate(db, page_size);

        let num_pages = paginator.num_items().await?;

        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }

    pub async fn add(db: &DbConn, data: Model) -> Result<Model, DbErr> {
        DataSourceListService::save(db, None, data).await
    }

    pub async fn update_by_id(db: &DbConn, id: i32, data: Model) -> Result<Model, DbErr> {
        DataSourceListService::save(db, Some(id), data).await
    }

    pub async fn save(db: &DbConn, id: Option<i32>, data: Model) -> Result<Model, DbErr> {
        debug!("data: {:?}, id: {:?}", data, id);
        let now = chrono::Local::now().naive_local();
        let mut active_data = data_source_list::ActiveModel {
            name: Set(data.name),
            database_name: Set(data.database_name),
            table_schema: Set(data.table_schema),
            database_type: Set(data.database_type),
            host: Set(data.host),
            port: Set(data.port),
            user: Set(data.user),
            password: Set(data.password),
            ..Default::default()
        };
        if let Some(id) = id {
            let db_data = data_source_list::Entity::find_by_id(id)
                .one(db)
                .await?
                .ok_or(DbErr::Custom("Cannot find data by id.".to_owned()))?;

            active_data.id = Unchanged(db_data.id);
            active_data.update_time = Set(now);
            active_data.update(db).await
        } else {
            active_data.create_time = Set(now);
            active_data.update_time = Set(now);
            active_data.insert(db).await
        }
    }


    pub async fn query_table_columns(
        data_source: DataSource,
        table_name: String,
    ) -> anyhow::Result<Vec<serde_json::Value>, DbErr> {
        let sql_string = match &data_source.database_type {
            process_core::db::Database::MYSQL => {
                format!(
                    "SELECT COLUMN_NAME 
                FROM INFORMATION_SCHEMA.COLUMNS 
                WHERE TABLE_SCHEMA = '{}' 
                AND TABLE_NAME = '{}';
                ",
                    &data_source.database_name, table_name
                )
            }
            process_core::db::Database::MSSQL => {
                format!(
                    "SELECT COLUMN_NAME 
                    FROM INFORMATION_SCHEMA.COLUMNS 
                    WHERE TABLE_NAME = '{}';",
                    table_name
                )
            }
            process_core::db::Database::POSTGRES | process_core::db::Database::KINGBASE => {
                format!(
                    "SELECT column_name 
                    FROM information_schema.columns 
                    WHERE table_schema = '{}' 
                    AND table_name = '{}';
                    ",
                    &data_source
                        .table_schema
                        .as_ref()
                        .unwrap_or(&"public".to_string()),
                    table_name
                )
            }
            process_core::db::Database::ORACLE => {
                format!(
                    "SELECT COLUMN_NAME 
                    FROM USER_TAB_COLUMNS 
                    WHERE TABLE_NAME = '{}';",
                    table_name
                )
            }
        };

        process_core::db::find_all_sql(&data_source, sql_string)
            .await
            .map_err(|err| {
                let s = format!("{}", err);
                DbErr::Custom(s.to_owned())
            })
    }

    pub async fn delete(db: &DbConn, id: i32) -> Result<Model, DbErr> {
        let data = data_source_list::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find data by id.".to_owned()))?;

        let mut active_data = data.into_active_model();

        active_data.del_flag = Set(1);

        active_data.update(db).await
    }
}
