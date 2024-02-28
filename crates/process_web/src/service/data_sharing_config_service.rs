use sea_orm::ActiveValue::{Set, Unchanged};
use sea_orm::*;
use tracing::debug;
use process_core::db::DataSource;

use crate::api::data_sharing_config::ListParams;
use crate::entity::{data_sharing_config};
use crate::entity::data_sharing_config::Model;

pub struct DataSharingConfigService;

impl DataSharingConfigService {
    pub async fn find_by_id(db: &DbConn, id: i32) -> Result<Model, DbErr> {
        data_sharing_config::Entity::find_by_id(id)
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
            if let Some(name) = data.name {
                conditions = conditions.add(data_sharing_config::Column::Name.contains(&name));
            }
        }

        let paginator = data_sharing_config::Entity::find()
            .filter(data_sharing_config::Column::DelFlag.eq(0))
            .filter(conditions)
            .order_by_desc(data_sharing_config::Column::Id)
            .paginate(db, page_size);

        let num_pages = paginator.num_items().await?;

        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }

    pub async fn add(db: &DbConn, data: Model) -> Result<Model, DbErr> {
        DataSharingConfigService::save(db, None, data).await
    }

    pub async fn update_by_id(db: &DbConn, id: i32, data: Model) -> Result<Model, DbErr> {
        DataSharingConfigService::save(db, Some(id), data).await
    }

    pub async fn save(db: &DbConn, id: Option<i32>, data: Model) -> Result<Model, DbErr> {
        debug!("data: {:?}, id: {:?}", data, id);
        let now = chrono::Local::now().naive_local();
        let mut active_data = data_sharing_config::ActiveModel {
            name: Set(data.name),
            table_name: Set(data.table_name),
            query_sql: Set(data.query_sql),
            data_source: Set(data.data_source),
            ..Default::default()
        };
        if let Some(id) = id {
            let db_data = data_sharing_config::Entity::find_by_id(id)
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


    pub async fn delete(db: &DbConn, id: i32) -> Result<Model, DbErr> {
        let data = data_sharing_config::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find data by id.".to_owned()))?;

        let mut active_data = data.into_active_model();

        active_data.del_flag = Set(1);

        active_data.update(db).await
    }

    pub async fn get_data(
        db: &DbConn,
        id: i32,
        payload: Option<serde_json::Value>
    ) -> anyhow::Result<Vec<serde_json::Value>, DbErr> {
        let data = data_sharing_config::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find data by id.".to_owned()))?;
        let query_sql = match payload {
            None => data.query_sql,
            Some(x) => {
                let obj = x.as_object().ok_or(DbErr::Custom("传递的参数无法解析".to_owned()))?;
                let mut sql = data.query_sql;
                for (key, value) in obj {
                    let p_key = format!("${{{key}}}");
                    if sql.contains(p_key.as_str()) {
                        sql = sql.replace(p_key.as_str(), value.to_string().as_str());
                    }
                }

                sql
            }
        };

        let data_source: DataSource = serde_json::from_value(data.data_source).unwrap();
        process_core::db::find_all_sql(&data_source, query_sql)
            .await
            .map_err(|err| {
                let s = format!("{}", err);
                DbErr::Custom(s.to_owned())
            })
    }
}
