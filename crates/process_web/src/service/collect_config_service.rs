use sea_orm::ActiveValue::{Set, Unchanged};
use sea_orm::*;
use tracing::debug;

use crate::entity::collect_config;

pub struct CollectConfigService;

impl CollectConfigService {
    pub async fn find_by_id(db: &DbConn, id: i32) -> Result<collect_config::Model, DbErr> {
        collect_config::Entity::find_by_id(id)
            .filter(collect_config::Column::DelFlag.eq(0))
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
            .filter(collect_config::Column::DelFlag.eq(0))
            .order_by_desc(collect_config::Column::Id)
            .paginate(db, page_size);

        let num_pages = paginator.num_items().await?;

        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }

    pub async fn add(
        db: &DbConn,
        cache_db: &DbConn,
        data: collect_config::Model,
    ) -> Result<collect_config::Model, DbErr> {
        CollectConfigService::save(db, cache_db, None, data).await
    }

    pub async fn update_by_id(
        db: &DbConn,
        cache_db: &DbConn,
        id: i32,
        data: collect_config::Model,
    ) -> Result<collect_config::Model, DbErr> {
        CollectConfigService::save(db, cache_db, Some(id), data).await
    }

    pub async fn save(
        db: &DbConn,
        cache_db: &DbConn,
        id: Option<i32>,
        data: collect_config::Model,
    ) -> Result<collect_config::Model, DbErr> {
        debug!("data: {:?}, id: {:?}", data, id);
        let now = chrono::Local::now().naive_utc();

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
            cache_table_name: Set(data.cache_table_name.clone()),
            max_number_of_result_data: Set(data.max_number_of_result_data),
            filed_of_result_data: Set(data.filed_of_result_data),
            max_count_of_request: Set(data.max_count_of_request),
            db_columns_config: Set(data.db_columns_config.clone()),
            ..Default::default()
        };
        if let Some(id) = id {
            let db_data = collect_config::Entity::find_by_id(id)
                .one(db)
                .await?
                .ok_or(DbErr::Custom("Cannot find data by id.".to_owned()))?;

            active_data.id = Unchanged(db_data.id);
            active_data.update_time = Set(now);

            if let Some(db_columns_config) = data.db_columns_config.as_ref() {
                if db_data.db_columns_config != data.db_columns_config {
                    Self::update_table_struct(cache_db, db_columns_config, data.cache_table_name.as_ref().unwrap()).await?;
                }
            }

            active_data.update(db).await
        } else {
            active_data.create_time = Set(now);
            active_data.update_time = Set(now);
            if let Some(db_columns_config) = data.db_columns_config.as_ref() {
                Self::create_table(cache_db, db_columns_config, data.cache_table_name.as_ref().unwrap()).await?;
            }
            active_data.insert(db).await
        }
    }

    pub async fn delete(db: &DbConn, id: i32) -> Result<collect_config::Model, DbErr> {
        let mut collect_config: collect_config::ActiveModel = collect_config::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find data by id.".to_owned()))
            .map(Into::into)?;

        collect_config.del_flag = Set(1);

        collect_config.update(db).await
    }

    pub async fn cache_data(cache_db: &DbConn, list: &Vec<String>) -> Result<bool, DbErr> {
        for item in list {
            cache_db.execute(
                Statement::from_string(
                    cache_db.get_database_backend(),
                    item
                )
            )
            .await?;
        }
        Ok(true)
    }

    pub async fn create_table(cache_db: &DbConn, db_columns_config: &serde_json::Value, table_name: &String,) -> Result<bool, DbErr> {
        if let Some(db_columns_config) = db_columns_config.as_array() {
            let mut template_str = format!("CREATE TABLE IF NOT EXISTS {table_name}");
            let mut column_str = vec![];
            for item in db_columns_config {
                column_str.push(format!("{} {} NULL", item["key"], item["type"]));
            }
            template_str = format!("{} ({});", template_str, column_str.join(", "));

            cache_db
                .execute(Statement::from_string(
                    cache_db.get_database_backend(),
                    template_str,
                ))
                .await?;
        } else {
            return Err(DbErr::Custom("db_columns_config无法解析为json数组".to_owned()));
        }

        Ok(true)
    }
    pub async fn update_table_struct(cache_db: &DbConn, db_columns_config: &serde_json::Value, table_name: &String,) -> Result<bool, DbErr> {
        let now = chrono::Local::now().naive_utc().timestamp();
        let alert_sql = format!("ALTER TABLE {table_name} rename to {table_name}_{now}");
        match cache_db
            .execute(Statement::from_string(cache_db.get_database_backend(), alert_sql,))
            .await {
            Err(err) => {
                // TODO 识别为表不存在的错误
                println!("{:?}", err);
            },
            _ => {},
        };

        Self::create_table(cache_db, db_columns_config, table_name).await
    }
}
