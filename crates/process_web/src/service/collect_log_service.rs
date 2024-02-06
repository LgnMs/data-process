use sea_orm::ActiveValue::{Set, Unchanged};
use sea_orm::*;
use tracing::debug;
use uuid::Uuid;
use crate::api::collect_log::ListParams;

use crate::entity::collect_config;
use crate::entity::collect_log;

pub struct CollectLogService;

impl CollectLogService {
    pub async fn find_by_id(db: &DbConn, id: Uuid) -> Result<collect_log::Model, DbErr> {
        collect_log::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find data by id.".to_owned()))
    }

    pub async fn list(
        db: &DbConn,
        page: u64,
        page_size: u64,
        data: Option<ListParams>
    ) -> Result<(Vec<serde_json::Value>, u64), DbErr> {

        let mut conditions = Condition::all();
        if let Some(data) = data {
            if let Some(name) = data.collect_config_name {
                conditions = conditions.add(collect_config::Column::Name.contains(&name));
            }
        }

        let db_res = collect_log::Entity::find()
            .find_also_related(collect_config::Entity)
            .offset((page - 1) * page_size)
            .limit(page_size)
            .filter(conditions)
            .order_by_desc(collect_log::Column::UpdateTime)
            .into_json()
            .all(db)
            .await?;

        let mut list = vec![];
        for (mut a, b) in db_res {
            a["collect_config"] = b.unwrap();
            list.push(a);
        }

        let num_pages = collect_log::Entity::find().all(db).await?.len() as u64;

        return Ok((list, num_pages));
    }

    pub async fn add(db: &DbConn, data: collect_log::Model) -> Result<collect_log::Model, DbErr> {
        CollectLogService::save(db, None, data).await
    }

    pub async fn update_by_id(
        db: &DbConn,
        id: Uuid,
        data: collect_log::Model,
    ) -> Result<collect_log::Model, DbErr> {
        CollectLogService::save(db, Some(id), data).await
    }

    pub async fn save(
        db: &DbConn,
        id: Option<Uuid>,
        data: collect_log::Model,
    ) -> Result<collect_log::Model, DbErr> {
        debug!("data: {:?}, id: {:?}", data, id);
        let now = chrono::Local::now().naive_local();
        let mut active_data = collect_log::ActiveModel {
            ..Default::default()
        };
        if let Some(id) = id {
            let db_data = collect_log::Entity::find_by_id(id)
                .one(db)
                .await?
                .ok_or(DbErr::Custom("Cannot find data by id.".to_owned()))?;

            active_data.id = Unchanged(db_data.id);
            active_data.status = Set(data.status);
            let log = format!(
                "{}\n{}",
                db_data.running_log.unwrap_or_default(),
                data.running_log.unwrap_or_default()
            );
            active_data.running_log = Set(Some(log));
            active_data.update_time = Set(now);
            active_data.update(db).await
        } else {
            active_data.id = Set(data.id);
            active_data.collect_config_id = Set(data.collect_config_id);
            active_data.status = Set(0);
            active_data.running_log = Set(Some(data.running_log.unwrap_or_default()));
            active_data.create_time = Set(now);
            active_data.update_time = Set(now);
            active_data.insert(db).await
        }
    }

    pub async fn delete(db: &DbConn, id: Uuid) -> Result<DeleteResult, DbErr> {
        let collect_log: collect_log::ActiveModel = collect_log::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find data by id.".to_owned()))
            .map(Into::into)?;

        collect_log.delete(db).await
    }
}
