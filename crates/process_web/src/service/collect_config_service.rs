use sea_orm::ActiveValue::{Set, Unchanged};
use sea_orm::*;
use tracing::debug;

use crate::entity::collect_config;

pub struct CollectConfigService;

impl CollectConfigService {
    pub async fn find_by_id(db: &DbConn, id: i32) -> Result<collect_config::Model, DbErr> {
        collect_config::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find data by id.".to_owned()))
    }

    pub async fn list(
        db: &DbConn,
        page: u64,
        page_size: u64,
    ) -> Result<(Vec<collect_config::Model>, u64), DbErr> {
        let paginator = collect_config::Entity::find()
            .order_by_asc(collect_config::Column::Id)
            .paginate(db, page_size);

        let num_pages = paginator.num_pages().await?;

        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }

    pub async fn add(
        db: &DbConn,
        data: collect_config::Model,
    ) -> Result<collect_config::Model, DbErr> {
        CollectConfigService::save(db, None, data).await
    }

    pub async fn update_by_id(
        db: &DbConn,
        id: i32,
        data: collect_config::Model,
    ) -> Result<collect_config::Model, DbErr> {
        CollectConfigService::save(db, Some(id), data).await
    }

    pub async fn save(
        db: &DbConn,
        id: Option<i32>,
        data: collect_config::Model,
    ) -> Result<collect_config::Model, DbErr> {
        debug!("data: {:?}, id: {:?}", data, id);
        let mut active_data = collect_config::ActiveModel {
            url: Set(data.url),
            name: Set(data.name),
            desc: Set(data.desc),
            method: Set(data.method),
            headers: Set(data.headers),
            body: Set(data.body),
            map_rules: Set(data.map_rules),
            template_string: Set(data.template_string),
            current_key: Set(data.current_key),
            page_size_key: Set(data.page_size_key),
            loop_request_by_pagination: Set(data.loop_request_by_pagination),
            cache_table_name: Set(data.cache_table_name),
            ..Default::default()
        };
        if let Some(id) = id {
            let db_data = collect_config::Entity::find_by_id(id)
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
        let collect_config: collect_config::ActiveModel = collect_config::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find data by id.".to_owned()))
            .map(Into::into)?;

        collect_config.delete(db).await
    }
}
