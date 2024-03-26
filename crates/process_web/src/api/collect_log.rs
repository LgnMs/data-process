use anyhow::Result;
use axum::extract::{Path, State};
use axum::Json;
use axum::{
    routing::{get, post},
    Router,
};
use sea_orm::prelude::DateTime;
use serde::Deserialize;
use std::sync::Arc;
use ts_rs::TS;

use crate::api::common::{AppError, AppState, PaginationPayload, ResJson, ResJsonWithPagination};
use crate::entity::collect_log::Model;
use crate::service::collect_log_service::CollectLogService;
use crate::{bool_response, data_response, pagination_response};

pub fn set_routes() -> Router<Arc<AppState>> {
    let routes = Router::new()
        .route("/find_by_id/:id", post(find_by_id))
        .route("/list", post(list))
        .route("/add", post(add))
        .route("/update_by_id/:id", post(update_by_id))
        .route("/delete/:id", get(del));

    routes
}

async fn find_by_id(
    state: State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<ResJson<Model>, AppError> {
    let res = CollectLogService::find_by_id(&state.conn, id).await;

    data_response!(res)
}

#[derive(Deserialize, TS)]
#[ts(
    export,
    export_to = "ui/api/models/auto-generates/CollectLogListParams.ts",
    rename = "CollectLogListParams"
)]
pub struct ListParams {
    pub collect_config_name: Option<String>,
    pub date: Option<[i64; 2]>,
}

async fn list(
    state: State<Arc<AppState>>,
    Json(payload): Json<PaginationPayload<ListParams>>,
) -> Result<ResJsonWithPagination<serde_json::Value>, AppError> {
    let res = CollectLogService::list(
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
    let res = CollectLogService::add(&state.conn, payload).await;

    data_response!(res)
}

async fn update_by_id(
    state: State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(payload): Json<Model>,
) -> Result<ResJson<Model>, AppError> {
    let res = CollectLogService::update_by_id(&state.conn, id, payload).await;

    data_response!(res)
}

async fn del(state: State<Arc<AppState>>, Path(id): Path<i32>) -> Result<ResJson<bool>, AppError> {
    let res = CollectLogService::delete(&state.conn, id).await;

    bool_response!(res)
}
