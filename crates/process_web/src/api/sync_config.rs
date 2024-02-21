use std::sync::Arc;
use axum::extract::{Path, State};
use axum::{Json, Router};
use axum::routing::{get, post};
use serde::Deserialize;
use ts_rs::TS;

use crate::api::common::*;
use crate::entity::sync_config::Model;
use crate::service::sync_config_service::SyncConfigService;

#[derive(Deserialize, TS)]
#[ts(
    export,
    export_to = "ui/api/models/auto-generates/SyncConfigListParams.ts",
    rename = "SyncConfigListParams"
)]
pub struct ListParams {
    pub name: Option<String>,
}

pub fn set_routes() -> Router<Arc<AppState>> {
    let routes = Router::new()
        .route("/find_by_id/:id", get(find_by_id))
        .route("/list", post(list))
        .route("/add", post(add))
        .route("/update_by_id/:id", post(update_by_id))
        .route("/del/:id", get(del))
        .route("/execute/:id", get(execute));

    routes
}

async fn find_by_id(
    state: State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> anyhow::Result<ResJson<Model>, AppError> {
    let res = SyncConfigService::find_by_id(&state.conn, id).await;

    data_response!(res)
}

async fn list(
    state: State<Arc<AppState>>,
    Json(payload): Json<PaginationPayload<ListParams>>,
) -> anyhow::Result<ResJsonWithPagination<Model>, AppError> {
    let res = SyncConfigService::list(
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
) -> anyhow::Result<ResJson<Model>, AppError> {
    let res = SyncConfigService::add(state.0, payload).await;

    data_response!(res)
}

async fn update_by_id(
    state: State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(payload): Json<Model>,
) -> anyhow::Result<ResJson<Model>, AppError> {
    let res = SyncConfigService::update_by_id(state.0, id, payload).await;

    data_response!(res)
}

async fn del(state: State<Arc<AppState>>, Path(id): Path<i32>) -> anyhow::Result<ResJson<bool>, AppError> {
    let res = SyncConfigService::delete(state.0, id).await;

    bool_response!(res)
}

/// 执行id所配置的采集任务
pub async fn execute(
    state: State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> anyhow::Result<ResJson<bool>, AppError> {
    let data = SyncConfigService::find_by_id(&state.conn, id).await?;

    tokio::task::spawn(async move {
        SyncConfigService::execute_task(&state, &data).await;
    });
    // https://docs.rs/tokio/1.35.1/tokio/task/index.html#yield_now
    // tokio::task::yield_now().await;

    Ok(Json(res_template_ok!(Some(true))))
}
