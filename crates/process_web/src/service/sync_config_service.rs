use anyhow::Result;
use chrono::Local;
use migration::Condition;
use process_core::db::{Db, DbConfig};
use process_core::process::{Export, Receive};
use sea_orm::prelude::*;
use sea_orm::ActiveValue::{Set, Unchanged};
use sea_orm::{IntoActiveModel, QueryOrder};
use std::sync::Arc;
use tokio_cron_scheduler::{Job, JobSchedulerError};
use tracing::{debug, error, warn};

use crate::api::common::AppState;
use crate::api::sync_config::ListParams;
use crate::entity::sync_config::Model;
use crate::entity::{sync_config, sync_log};
use crate::service::sync_log_service::SyncLogService;
use crate::utils::{format_cron, job_err_to_db_err};

pub struct SyncConfigService;

impl SyncConfigService {
    pub async fn find_by_id(db: &DbConn, id: i32) -> Result<Model, DbErr> {
        sync_config::Entity::find_by_id(id)
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
                conditions = conditions.add(sync_config::Column::Name.contains(&name));
            }
        }

        let paginator = sync_config::Entity::find()
            .filter(sync_config::Column::DelFlag.eq(0))
            .filter(conditions)
            .order_by_desc(sync_config::Column::Id)
            .paginate(db, page_size);

        let num_pages = paginator.num_items().await?;

        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }

    pub async fn add(state: Arc<AppState>, data: Model) -> std::result::Result<Model, DbErr> {
        Self::save(state, None, data).await
    }

    pub async fn update_by_id(
        state: Arc<AppState>,
        id: i32,
        data: Model,
    ) -> std::result::Result<Model, DbErr> {
        Self::save(state, Some(id), data).await
    }

    pub async fn save(state: Arc<AppState>, id: Option<i32>, data: Model) -> Result<Model, DbErr> {
        debug!("data: {:?}, id: {:?}", data, id);
        let now = Local::now().naive_local();

        let data_clone = data.clone();
        let mut active_data = sync_config::ActiveModel {
            name: Set(data_clone.name),
            data_source: Set(data_clone.data_source),
            source_table_name: Set(data_clone.source_table_name),
            source_table_columns: Set(data_clone.source_table_columns),
            query_sql: Set(data_clone.query_sql),
            target_data_source: Set(data_clone.target_data_source),
            target_table_name: Set(data_clone.target_table_name),
            target_query_sql_template: Set(data_clone.target_query_sql_template),
            cron: Set(data_clone.cron),
            ..Default::default()
        };

        if let Some(id) = id {
            // 更新
            let db_data = sync_config::Entity::find_by_id(id)
                .one(&state.conn)
                .await?
                .ok_or(DbErr::Custom("Cannot find data by id.".to_owned()))?;

            active_data.id = Unchanged(db_data.id);
            active_data.update_time = Set(now);

            let job_id = update_job_scheduler(state.clone(), &data, &db_data)
                .await
                .map_err(job_err_to_db_err)?;
            if let Some(new_job_id) = job_id {
                active_data.job_id = Set(Some(new_job_id));
            }

            active_data.update(&state.conn).await
        } else {
            active_data.create_time = Set(now);
            active_data.update_time = Set(now);

            let job_id = create_job_scheduler(state.clone(), &data)
                .await
                .map_err(job_err_to_db_err)?;
            active_data.job_id = Set(job_id);

            active_data.insert(&state.conn).await
        }
    }

    pub async fn delete(state: Arc<AppState>, id: i32) -> Result<Model, DbErr> {
        let data = sync_config::Entity::find_by_id(id)
            .one(&state.conn)
            .await?
            .ok_or(DbErr::Custom("Cannot find data by id.".to_owned()))?;

        if let Some(job_id) = data.job_id {
            state
                .sched
                .remove(&job_id)
                .await
                .map_err(job_err_to_db_err)?;
        }
        let mut active_data = data.into_active_model();

        active_data.del_flag = Set(1);
        active_data.job_id = Set(None);

        active_data.update(&state.conn).await
    }

    pub async fn update_job_id_by_id(
        state: Arc<AppState>,
        job_id: Option<Uuid>,
        id: i32,
    ) -> Result<Model, DbErr> {
        let mut db_data = sync_config::Entity::find_by_id(id)
            .one(&state.conn)
            .await?
            .ok_or(DbErr::Custom("Cannot find data by id.".to_owned()))?
            .into_active_model();

        db_data.job_id = Set(job_id);

        db_data.update(&state.conn).await
    }

    pub async fn execute_task(state: &Arc<AppState>, data: &Model) {
        let sync_log_id = Uuid::new_v4();
        let sync_log_model = sync_log::Model {
            id: sync_log_id,
            sync_config_id: data.id,
            status: 0,
            running_log: String::new(),
            ..Default::default()
        };

        if let Some(err) = SyncLogService::add(&state.conn, sync_log_model).await.err() {
            error!("任务日志添加失败 {err}");
        };

        let mut status = 1;
        let model = sync_log::Model {
            status,
            running_log: "开始执行采集任务!".to_string(),
            ..Default::default()
        };
        if let Some(err) = SyncLogService::update_by_id(&state.conn, sync_log_id, model)
            .await
            .err()
        {
            error!("status: {status} 运行中；日志更新失败: {err}");
        };

        let mut collect_log_string = String::new();

        collect_log_string.push_str(format!("同步配置： {:?}\n", data).as_str());
        let res = process_data(&data).await;
        match res {
            Ok(list) => {
                collect_log_string.push_str(format!("SQL执行成功： {:?}\n", list).as_str());
                status = 2;
                collect_log_string.push_str("同步任务执行成功!\n");
            }
            Err(err) => {
                let err_str = format!("{}\n", err);
                collect_log_string.push_str(err_str.as_str());
                status = 3;
                error!("status: {status} 运行失败；日志更新失败: {err_str}");
            }
        }

        let model = sync_log::Model {
            status,
            running_log: collect_log_string,
            ..Default::default()
        };
        if let Some(err) = SyncLogService::update_by_id(&state.conn, sync_log_id, model)
            .await
            .err()
        {
            error!("status: {status} 运行完毕；日志更新失败: {err}");
        };
    }

    /// 初始化所有的同步调度任务
    pub async fn setup_collect_config_cron(state: &Arc<AppState>) -> anyhow::Result<()> {
        let list = sync_config::Entity::find()
            .filter(sync_config::Column::DelFlag.eq(0))
            .all(&state.conn)
            .await?;

        for item in list {
            let state = state.clone();
            let item = item.clone();
            let job_id = create_job_scheduler(state.clone(), &item).await?;

            Self::update_job_id_by_id(state, job_id, item.id).await?;
        }
        state.sched.start().await?;

        Ok(())
    }
}

async fn process_data(data: &Model) -> Result<Vec<String>> {
    let mut db = Db::new();

    db.receive(
        data.query_sql.clone(),
        DbConfig {
            db_source_config: serde_json::from_value(data.data_source.clone())?,
        },
    )
    .await?
    .set_template_string(data.target_query_sql_template.clone())
    .set_target_db_source_config(serde_json::from_value(data.target_data_source.clone())?)
    .export()
    .await
}

async fn update_job_scheduler(
    state: Arc<AppState>,
    data: &Model,
    db_data: &Model,
) -> Result<Option<Uuid>, JobSchedulerError> {
    if db_data.cron != data.cron {
        if let Some(job_id) = db_data.job_id {
            state.sched.remove(&job_id).await?;
        }
        return create_job_scheduler(state, data).await;
    }
    Ok(None)
}

async fn create_job_scheduler(
    state: Arc<AppState>,
    data: &Model,
) -> Result<Option<Uuid>, JobSchedulerError> {
    if let Some(cron) = data.cron.as_ref() {
        let sched_state = state.clone();
        let data = data.clone();
        let job_id = state
            .sched
            .add(Job::new_async(
                format_cron(cron.clone()).as_str(),
                move |_uuid, mut _l| {
                    let st = sched_state.clone();
                    let item_c = data.clone();
                    Box::pin(async move {
                        SyncConfigService::execute_task(&st, &item_c).await;
                    })
                },
            )?)
            .await?;
        return Ok(Some(job_id));
    } else {
        let err = format!(
            "同步配置：{} 定时任务添加失败 cron: {:?} ",
            data.name, data.cron
        );
        warn!("{}", err);
    }
    Ok(None)
}
