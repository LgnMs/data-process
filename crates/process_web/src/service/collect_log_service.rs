use crate::api::collect_log::ListParams;
use chrono::{Local, TimeZone};
use sea_orm::ActiveValue::{Set, Unchanged};
use sea_orm::*;
use tracing::debug;

use crate::entity::collect_config;
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
        data: Option<ListParams>,
    ) -> Result<(Vec<serde_json::Value>, u64), DbErr> {
        let mut conditions = Condition::all();
        if let Some(data) = data {
            if let Some(name) = data.collect_config_name {
                conditions = conditions.add(collect_config::Column::Name.contains(name));
            }
            if let Some([start_date, end_date]) = data.date {
                conditions = conditions
                    .add(
                        collect_config::Column::UpdateTime.gte(
                            Local
                                .timestamp_millis_opt(start_date)
                                .unwrap()
                                .naive_local(),
                        ),
                    )
                    .add(
                        collect_config::Column::UpdateTime
                            .lte(Local.timestamp_millis_opt(end_date).unwrap().naive_local()),
                    );
            }
        }
        let select = collect_log::Entity::find()
            .select_only()
            .column(collect_config::Column::Name)
            .columns(collect_log::Column::iter().filter(|col| match col {
                collect_log::Column::RunningLog => false,
                _ => true,
            }))
            .inner_join(collect_config::Entity)
            .offset((page - 1) * page_size)
            .limit(page_size)
            .filter(conditions.clone())
            .order_by_desc(collect_log::Column::UpdateTime)
            .into_json();

        let db_res = select.paginate(db, page_size);

        let num_pages = db_res.num_items().await?;

        match db_res.fetch_page(page - 1).await {
            Ok(p) => Ok((p, num_pages)),
            Err(err) => {
                println!("err {err}");
                Err(err)
            }
        }
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
        let now = chrono::Local::now().naive_local();
        let mut active_data = collect_log::ActiveModel {
            ..Default::default()
        };
        if data.task_id.is_some() {
            active_data.task_id = Set(data.task_id);
        }

        if let Some(id) = id {
            let db_data = collect_log::Entity::find_by_id(id)
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
            active_data.collect_config_id = Set(data.collect_config_id);
            active_data.status = Set(0);
            active_data.running_log = Set(data.running_log);
            active_data.create_time = Set(now);
            active_data.update_time = Set(now);
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
