use sea_orm::ActiveValue::{Set, Unchanged};
use sea_orm::*;
use tracing::debug;

use crate::entity::collect_log;

pub struct CollectLogService;

impl CollectLogService {
    pub async fn find_by_id(db: &DbConn, id: i32) -> Result<collect_log::Model, DbErr> {
        collect_log::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find data by id.".to_owned()))
    }

    pub async fn list(
        db: &DbConn,
        page: u64,
        page_size: u64,
    ) -> Result<(Vec<collect_log::Model>, u64), DbErr> {
        let paginator = collect_log::Entity::find()
            .order_by_asc(collect_log::Column::Id)
            .paginate(db, page_size);

        let num_pages = paginator.num_pages().await?;

        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }

    pub async fn add(db: &DbConn, data: collect_log::Model) -> Result<collect_log::Model, DbErr> {
        CollectLogService::save(db, None, data).await
    }

    pub async fn update_by_id(
        db: &DbConn,
        id: i32,
        data: collect_log::Model,
    ) -> Result<collect_log::Model, DbErr> {
        CollectLogService::save(db, Some(id), data).await
    }

    pub async fn save(
        db: &DbConn,
        id: Option<i32>,
        data: collect_log::Model,
    ) -> Result<collect_log::Model, DbErr> {
        debug!("data: {:?}, id: {:?}", data, id);
        let mut active_data = collect_log::ActiveModel {
            running_log: Set(data.running_log),
            collect_config_id: Set(data.collect_config_id),
            ..Default::default()
        };
        if let Some(id) = id {
            let db_data = collect_log::Entity::find_by_id(id)
                .one(db)
                .await?
                .ok_or(DbErr::Custom("Cannot find data by id.".to_owned()))?;

            active_data.id = Unchanged(db_data.id);
            active_data.update(db).await
        } else {
            active_data.insert(db).await
        }
    }

    pub async fn delete(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        let collect_log: collect_log::ActiveModel = collect_log::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find data by id.".to_owned()))
            .map(Into::into)?;

        collect_log.delete(db).await
    }
}
