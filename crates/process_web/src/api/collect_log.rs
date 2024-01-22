use anyhow::Result;
use axum::extract::{Path, State};
use axum::{http::StatusCode, Json};
use axum::{
    routing::{get, post},
    Router,
};
use schemars::JsonSchema;
use serde::Deserialize;
use std::sync::Arc;

use crate::api::common::{
    AppError, AppState, Id, Pagination, ResJson, ResJsonWithPagination, ResTemplate,
};
use crate::entity::collect_log::Model;
use crate::service::collect_log_service::CollectLogService;

pub fn set_routes() -> Router<Arc<AppState>> {
    let routes = Router::new()
        .route("/find_by_id/:id", post(find_by_id))
        .route("/list", post(list))
        .route("/add", post(add))
        .route("/update_by_id/:id", post(update_by_id))
        .route("/delete/:id", get(delete));

    routes
}

#[derive(Deserialize, JsonSchema)]
struct QueryList {
    current: u64,
    page_size: u64,
}

async fn find_by_id(
    state: State<Arc<AppState>>,
    Path(id): Path<Id>,
) -> Result<ResJson<Model>, AppError> {
    let res = CollectLogService::find_by_id(&state.conn, id.id).await;

    data_response!(res)
}

async fn list(
    state: State<Arc<AppState>>,
    Json(payload): Json<QueryList>,
) -> Result<ResJsonWithPagination<Model>, AppError> {
    let res = CollectLogService::list(&state.conn, payload.current, payload.page_size).await;

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
    Path(id): Path<Id>,
    Json(payload): Json<Model>,
) -> Result<ResJson<Model>, AppError> {
    let res = CollectLogService::update_by_id(&state.conn, id.id, payload).await;

    data_response!(res)
}

async fn delete(
    state: State<Arc<AppState>>,
    Path(id): Path<Id>,
) -> Result<ResJson<bool>, AppError> {
    let res = CollectLogService::delete(&state.conn, id.id).await;

    bool_response!(res)
}
