use crate::api::sharing_request_log::ListParams;
use crate::entity::{data_sharing_config, sharing_request_log};
use sea_orm::ActiveValue::{Set, Unchanged};
use sea_orm::*;
use serde_json::json;
use tracing::debug;

pub struct SharingRequestLogService;

impl SharingRequestLogService {
    pub async fn find_by_id(db: &DbConn, id: i32) -> Result<sharing_request_log::Model, DbErr> {
        sharing_request_log::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find data by id.".to_owned()))
    }

    pub async fn list(
        db: &DbConn,
        page: u64,
        page_size: u64,
        data: Option<ListParams>,
    ) -> Result<(Vec<serde_json::Value>, u64), DbErr> {
        let mut conditions = Condition::all();
        if let Some(data) = data {
            if let Some(name) = data.data_sharing_config_name {
                conditions = conditions.add(data_sharing_config::Column::Name.contains(&name));
            }
        }

        let db_res = sharing_request_log::Entity::find()
            .find_also_related(data_sharing_config::Entity)
            .offset((page - 1) * page_size)
            .limit(page_size)
            .filter(conditions)
            .order_by_desc(sharing_request_log::Column::UpdateTime)
            .all(db)
            .await?;

        let mut list = vec![];
        for (a, b) in db_res {
            let mut a = json!(a);
            a["data_sharing_config"] = json!(b.unwrap_or_default());
            list.push(a);
        }

        let num_pages = sharing_request_log::Entity::find().all(db).await?.len() as u64;

        return Ok((list, num_pages));
    }

    pub async fn add(
        db: &DbConn,
        data: sharing_request_log::Model,
    ) -> Result<sharing_request_log::Model, DbErr> {
        SharingRequestLogService::save(db, None, data).await
    }

    pub async fn update_by_id(
        db: &DbConn,
        id: i32,
        data: sharing_request_log::Model,
    ) -> Result<sharing_request_log::Model, DbErr> {
        SharingRequestLogService::save(db, Some(id), data).await
    }

    pub async fn save(
        db: &DbConn,
        id: Option<i32>,
        data: sharing_request_log::Model,
    ) -> Result<sharing_request_log::Model, DbErr> {
        debug!("data: {:?}, id: {:?}", data, id);
        let now = chrono::Local::now().naive_local();
        let mut active_data = sharing_request_log::ActiveModel {
            data_sharing_config_id: Set(data.data_sharing_config_id),
            ..Default::default()
        };
        if let Some(id) = id {
            let db_data = sharing_request_log::Entity::find_by_id(id)
                .one(db)
                .await?
                .ok_or(DbErr::Custom("Cannot find data by id.".to_owned()))?;

            active_data.id = Unchanged(db_data.id);
            let log = format!("{}\n{}", db_data.log, data.log);
            active_data.log = Set(log);
            active_data.update_time = Set(now);
            active_data.update(db).await
        } else {
            active_data.log = Set(data.log);
            active_data.create_time = Set(now);
            active_data.update_time = Set(now);
            active_data.insert(db).await
        }
    }

    pub async fn delete(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        let sharing_request_log: sharing_request_log::ActiveModel =
            sharing_request_log::Entity::find_by_id(id)
                .one(db)
                .await?
                .ok_or(DbErr::Custom("Cannot find data by id.".to_owned()))
                .map(Into::into)?;

        sharing_request_log.delete(db).await
    }
}
