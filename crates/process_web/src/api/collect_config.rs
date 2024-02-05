use std::sync::Arc;

use anyhow::Result;
use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use tracing::error;
use uuid::Uuid;

use crate::api::common::{
    AppError, AppState, Pagination, PaginationPayload, ResJson, ResJsonWithPagination, ResTemplate,
};
use crate::entity::collect_config::Model;
use crate::entity::collect_log;
use crate::service::collect_config_service::CollectConfigService;
use crate::service::collect_log_service::CollectLogService;

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
) -> Result<ResJson<Model>, AppError> {
    let res = CollectConfigService::find_by_id(&state.conn, id).await;

    data_response!(res)
}

async fn list(
    state: State<Arc<AppState>>,
    Json(payload): Json<PaginationPayload<Model>>,
) -> Result<ResJsonWithPagination<Model>, AppError> {
    let res = CollectConfigService::list(&state.conn, payload.current, payload.page_size).await;

    pagination_response!(res, payload.current, payload.page_size)
}
async fn add(
    state: State<Arc<AppState>>,
    Json(payload): Json<Model>,
) -> Result<ResJson<Model>, AppError> {
    let res = CollectConfigService::add(&state.conn, &state.cache_conn, payload).await;

    data_response!(res)
}

async fn update_by_id(
    state: State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(payload): Json<Model>,
) -> Result<ResJson<Model>, AppError> {
    let res = CollectConfigService::update_by_id(&state.conn, &state.cache_conn, id, payload).await;

    data_response!(res)
}

async fn del(state: State<Arc<AppState>>, Path(id): Path<i32>) -> Result<ResJson<bool>, AppError> {
    let res = CollectConfigService::delete(&state.conn, id).await;

    bool_response!(res)
}

/// 执行id所配置的采集任务
pub async fn execute(
    state: State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<ResJson<bool>, AppError> {
    let data = CollectConfigService::find_by_id(&state.conn, id).await?;

    tokio::task::spawn(async move {
        CollectConfigService::execute_task(&state, &data).await;
    });
    // https://docs.rs/tokio/1.35.1/tokio/task/index.html#yield_now
    // tokio::task::yield_now().await;

    Ok(Json(res_template_ok!(Some(true))))
}
