use sea_orm::ActiveValue::{Set, Unchanged};
use sea_orm::*;
use tracing::debug;
use uuid::Uuid;

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
    ) -> Result<(Vec<collect_log::Model>, u64), DbErr> {
        let paginator = collect_log::Entity::find()
            .order_by_asc(collect_log::Column::Id)
            .paginate(db, page_size);

        let num_pages = paginator.num_items().await?;

        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
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
        let now = chrono::Local::now().naive_utc();
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
            let log = format!("{}\n{}", db_data.running_log.unwrap_or_default(), data.running_log.unwrap_or_default());
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
