use aide::axum::{
    routing::{get, post},
    ApiRouter,
};
use anyhow::Result;
use axum::extract::{Path, State};
use axum::{http::StatusCode, Json};
use schemars::JsonSchema;
use serde::Deserialize;
use std::sync::Arc;

use crate::api::common::{AppState, Id, Pagination, ResJson, ResJsonWithPagination, ResTemplate};
use crate::entity::collect_log::Model;
use crate::service::collect_log_service::CollectLogService;

pub fn set_routes() -> ApiRouter<Arc<AppState>> {
    let routes = ApiRouter::new()
        .api_route("/find_by_id/:id", post(find_by_id))
        .api_route("/list", post(list))
        .api_route("/add", post(add))
        .api_route("/update_by_id/:id", post(update_by_id))
        .api_route("/delete/:id", get(delete));

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
) -> Result<ResJson<Model>, (StatusCode, String)> {
    let res = CollectLogService::find_by_id(&state.conn, id.id).await;

    data_response!(res)
}

async fn list(
    state: State<Arc<AppState>>,
    Json(payload): Json<QueryList>,
) -> Result<ResJsonWithPagination<Model>, (StatusCode, String)> {
    let res = CollectLogService::list(&state.conn, payload.current, payload.page_size).await;

    pagination_response!(res, payload.current, payload.page_size)
}
async fn add(
    state: State<Arc<AppState>>,
    Json(payload): Json<Model>,
) -> Result<ResJson<Model>, (StatusCode, String)> {
    let res = CollectLogService::add(&state.conn, payload).await;

    data_response!(res)
}

async fn update_by_id(
    state: State<Arc<AppState>>,
    Path(id): Path<Id>,
    Json(payload): Json<Model>,
) -> Result<ResJson<Model>, (StatusCode, String)> {
    let res = CollectLogService::update_by_id(&state.conn, id.id, payload).await;

    data_response!(res)
}

async fn delete(
    state: State<Arc<AppState>>,
    Path(id): Path<Id>,
) -> Result<ResJson<bool>, (StatusCode, String)> {
    let res = CollectLogService::delete(&state.conn, id.id).await;

    bool_response!(res)
}
