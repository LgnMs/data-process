use anyhow::Result;
use axum::extract::{Path, State};
use axum::Json;
use axum::{
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use std::sync::Arc;
use tracing::error;
use ts_rs::TS;
use uuid::Uuid;

use crate::api::common::{AppError, AppState, PaginationPayload, ResJson, ResJsonWithPagination};
use crate::entity::collect_log::{self, Model};
use crate::service::collect_log_service::CollectLogService;
use crate::{bool_response, data_response, pagination_response};

pub fn set_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/find_by_id/:id", get(find_by_id))
        .route("/list", post(list))
        .route("/add", post(add))
        .route("/update_by_id/:id", post(update_by_id))
        .route("/delete/:id", get(del))
        .route("/stop_task/:id", get(stop_task))
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

pub async fn stop_task(
    state: State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<ResJson<bool>, AppError> {
    let log_task_id = Uuid::parse_str(id.as_str())?.simple();

    let log_id = state.stop_log_task(log_task_id).await;
    if let Some(log_id) = log_id {
        if let Err(err) = CollectLogService::update_by_id(
            &state.conn,
            log_id,
            collect_log::Model {
                status: 5,
                running_log: "用户手动停止".to_string(),
                ..Default::default()
            },
        )
        .await
        {
            error!("{}", err);
        }
    }

    bool_response!(anyhow::Ok::<bool>(true))
}
