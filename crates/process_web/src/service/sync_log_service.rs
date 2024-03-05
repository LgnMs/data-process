use crate::api::sync_log::ListParams;
use crate::entity::{sync_config, sync_log};
use sea_orm::ActiveValue::{Set, Unchanged};
use sea_orm::*;
use tracing::debug;

pub struct SyncLogService;

impl SyncLogService {
    pub async fn find_by_id(db: &DbConn, id: i32) -> Result<sync_log::Model, DbErr> {
        sync_log::Entity::find_by_id(id)
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
            if let Some(name) = data.sync_config_name {
                conditions = conditions.add(sync_config::Column::Name.contains(&name));
            }
        }

        let db_res = sync_log::Entity::find()
            .find_also_related(sync_config::Entity)
            .offset((page - 1) * page_size)
            .limit(page_size)
            .filter(conditions)
            .order_by_desc(sync_log::Column::UpdateTime)
            .into_json()
            .all(db)
            .await?;

        let mut list = vec![];
        for (mut a, b) in db_res {
            a["sync_config"] = b.unwrap();
            list.push(a);
        }

        let num_pages = sync_log::Entity::find().all(db).await?.len() as u64;

        return Ok((list, num_pages));
    }

    pub async fn add(db: &DbConn, data: sync_log::Model) -> Result<sync_log::Model, DbErr> {
        SyncLogService::save(db, None, data).await
    }

    pub async fn update_by_id(
        db: &DbConn,
        id: i32,
        data: sync_log::Model,
    ) -> Result<sync_log::Model, DbErr> {
        SyncLogService::save(db, Some(id), data).await
    }

    pub async fn save(
        db: &DbConn,
        id: Option<i32>,
        data: sync_log::Model,
    ) -> Result<sync_log::Model, DbErr> {
        debug!("data: {:?}, id: {:?}", data, id);
        let now = chrono::Local::now().naive_local();
        let mut active_data = sync_log::ActiveModel {
            ..Default::default()
        };
        if let Some(id) = id {
            let db_data = sync_log::Entity::find_by_id(id)
                .one(db)
                .await?
                .ok_or(DbErr::Custom("Cannot find data by id.".to_owned()))?;

            active_data.id = Unchanged(db_data.id);
            active_data.status = Set(data.status);
            let log = format!("{}\n{}", db_data.running_log, data.running_log);
            active_data.running_log = Set(log);
            active_data.update_time = Set(now);
            active_data.update(db).await
        } else {
            active_data.sync_config_id = Set(data.sync_config_id);
            active_data.status = Set(0);
            active_data.running_log = Set(data.running_log);
            active_data.create_time = Set(now);
            active_data.update_time = Set(now);
            active_data.insert(db).await
        }
    }

    pub async fn delete(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        let sync_log: sync_log::ActiveModel = sync_log::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find data by id.".to_owned()))
            .map(Into::into)?;

        sync_log.delete(db).await
    }
}
