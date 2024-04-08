use std::sync::Arc;

use anyhow::Result;
use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use tracing::debug;
use ts_rs::TS;
use uuid::Uuid;

use crate::api::common::{AppError, AppState, LogTask, PaginationPayload, ResJson, ResJsonWithPagination};
use crate::entity::collect_config::Model;
use crate::service::collect_config_service::CollectConfigService;
use crate::{bool_response, data_response, pagination_response};

pub fn set_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/find_by_id/:id", get(find_by_id))
        .route("/list", post(list))
        .route("/add", post(add))
        .route("/update_by_id/:id", post(update_by_id))
        .route("/del/:id", get(del))
        .route("/execute/:id", get(execute))
}

async fn find_by_id(
    state: State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<ResJson<Model>, AppError> {
    let res = CollectConfigService::find_by_id(&state.conn, id).await;

    data_response!(res)
}

#[derive(Deserialize, TS)]
#[ts(
    export,
    export_to = "ui/api/models/auto-generates/CollectConfigListParams.ts",
    rename = "CollectConfigListParams"
)]
pub struct ListParams {
    pub name: Option<String>,
}

async fn list(
    state: State<Arc<AppState>>,
    Json(payload): Json<PaginationPayload<ListParams>>,
) -> Result<ResJsonWithPagination<Model>, AppError> {
    let res = CollectConfigService::list(
        &state.conn,
        payload.current,
        payload.page_size,
        payload.data,
    )
    .await;

    pagination_response!(res, payload.current, payload.page_size)
}
async fn add(
    state: State<Arc<AppState>>,
    Json(payload): Json<Model>,
) -> Result<ResJson<Model>, AppError> {
    let res = CollectConfigService::add(state.0, payload).await;

    data_response!(res)
}

async fn update_by_id(
    state: State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(payload): Json<Model>,
) -> Result<ResJson<Model>, AppError> {
    let res = CollectConfigService::update_by_id(state.0, id, payload).await;

    data_response!(res)
}

async fn del(state: State<Arc<AppState>>, Path(id): Path<i32>) -> Result<ResJson<bool>, AppError> {
    let res = CollectConfigService::delete(state.0, id).await;

    bool_response!(res)
}

/// 执行id所配置的采集任务
pub async fn execute(
    state: State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<ResJson<bool>, AppError> {
    let data = CollectConfigService::find_by_id(&state.conn, id).await?;

    let task_id = Uuid::new_v4().simple();
    let st = state.clone();
    let t_id = task_id.clone();
    let log_task = LogTask::new();
    let cloned_token = log_task.token.clone();
    let mut task = state.log_task.write().await;
    task.insert(task_id, log_task);
    drop(task);

    tokio::task::spawn(async move {
        tokio::select! {
            _ = cloned_token.cancelled() => {
                // The token was cancelled, task can shut down
                debug!("cloned_token {cloned_token:?}");
            }
            _ = CollectConfigService::execute_task(&st, &data, t_id) => {}
        }
    });
    let res: anyhow::Result<bool> = Ok(true);
    bool_response!(res)
}
