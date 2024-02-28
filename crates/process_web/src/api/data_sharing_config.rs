use crate::api::common::{AppError, AppState, PaginationPayload, ResJson, ResJsonWithPagination};
use crate::{bool_response, data_response, pagination_response};
use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Deserialize;
use std::sync::Arc;
use serde_json::Value;
use ts_rs::TS;

use crate::entity::data_sharing_config::Model;
use crate::service::data_sharing_config_service::DataSharingConfigService;

pub fn set_routes() -> Router<Arc<AppState>> {
    let routes = Router::new()
        .route("/find_by_id/:id", get(find_by_id))
        .route("/list", post(list))
        .route("/add", post(add))
        .route("/update_by_id/:id", post(update_by_id))
        .route("/get_data/:id", post(get_data))
        .route("/del/:id", get(del));

    routes
}

#[derive(Deserialize, TS)]
#[ts(
    export,
    export_to = "ui/api/models/auto-generates/DataSharingConfigParams.ts",
    rename = "DataSharingConfigParams"
)]
pub struct ListParams {
    pub name: Option<String>,
}

async fn find_by_id(
    state: State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> anyhow::Result<ResJson<Model>, AppError> {
    let res = DataSharingConfigService::find_by_id(&state.conn, id).await;
    data_response!(res)
}

async fn list(
    state: State<Arc<AppState>>,
    Json(payload): Json<PaginationPayload<ListParams>>,
) -> anyhow::Result<ResJsonWithPagination<Model>, AppError> {
    let res = DataSharingConfigService::list(
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
    let res = DataSharingConfigService::add(&state.conn, payload).await;

    data_response!(res)
}

async fn update_by_id(
    state: State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(payload): Json<Model>,
) -> anyhow::Result<ResJson<Model>, AppError> {
    let res = DataSharingConfigService::update_by_id(&state.conn, id, payload).await;

    data_response!(res)
}

async fn del(
    state: State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> anyhow::Result<ResJson<bool>, AppError> {
    let res = DataSharingConfigService::delete(&state.conn, id).await;

    bool_response!(res)
}

/// 通过共享配置的id执行查询语句并返回，payload中的数据会作为参数替换查询语句中的${xxx}
/// 例如
/// ```shell
/// curl --location 'http://127.0.0.1:8000/data_sharing_config/get_data/1' \
/// --header 'Content-Type: application/json' \
/// --data '{
///     "id": 1,
///     "limit": 5
/// }'
/// ```
/// ```SQL
///  原始语句：
///  select id, from public.test_data where id > ${id} limit ${limit};
///  输出：
///  select id, from public.test_data where id > 1 limit 5;
/// ```
/// 
async fn get_data(
    state: State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(payload): Json<Option<Value>>,
) -> anyhow::Result<ResJson<Vec<Value>>, AppError> {
    let res = DataSharingConfigService::get_data(&state.conn, id, payload).await;

    data_response!(res)
}