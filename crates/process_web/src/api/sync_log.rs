use anyhow::Result;
use axum::extract::{Path, State};
use axum::Json;
use axum::{
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use std::sync::Arc;
use ts_rs::TS;

use crate::api::common::{AppError, AppState, PaginationPayload, ResJson, ResJsonWithPagination};
use crate::entity::sync_log::Model;
use crate::service::sync_log_service::SyncLogService;
use crate::{bool_response, data_response, pagination_response};

pub fn set_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/find_by_id/:id", get(find_by_id))
        .route("/list", post(list))
        .route("/add", post(add))
        .route("/update_by_id/:id", post(update_by_id))
        .route("/delete/:id", get(del))
}

async fn find_by_id(
    state: State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<ResJson<Model>, AppError> {
    let res = SyncLogService::find_by_id(&state.conn, id).await;

    data_response!(res)
}

#[derive(Deserialize, TS)]
#[ts(
    export,
    export_to = "ui/api/models/auto-generates/SyncLogListParams.ts",
    rename = "SyncLogListParams"
)]
pub struct ListParams {
    pub sync_config_name: Option<String>,
}

async fn list(
    state: State<Arc<AppState>>,
    Json(payload): Json<PaginationPayload<ListParams>>,
) -> Result<ResJsonWithPagination<serde_json::Value>, AppError> {
    let res = SyncLogService::list(
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
    let res = SyncLogService::add(&state.conn, payload).await;

    data_response!(res)
}

async fn update_by_id(
    state: State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(payload): Json<Model>,
) -> Result<ResJson<Model>, AppError> {
    let res = SyncLogService::update_by_id(&state.conn, id, payload).await;

    data_response!(res)
}

async fn del(state: State<Arc<AppState>>, Path(id): Path<i32>) -> Result<ResJson<bool>, AppError> {
    let res = SyncLogService::delete(&state.conn, id).await;

    bool_response!(res)
}
