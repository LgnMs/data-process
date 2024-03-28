use crate::api::common::{AppError, AppState, PaginationPayload, ResJson, ResJsonWithPagination};
use crate::entity::data_source_list::Model;
use crate::service::data_source_list_service::DataSourceListService;
use crate::{bool_response, data_response, pagination_response};
use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use process_core::db::DataSource;
use serde::Deserialize;
use serde_json::Value;
use std::sync::Arc;
use ts_rs::TS;

pub fn set_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/find_by_id/:id", get(find_by_id))
        .route("/list", post(list))
        .route("/add", post(add))
        .route("/update_by_id/:id", post(update_by_id))
        .route("/query_table_columns", post(query_table_columns))
        .route("/del/:id", get(del))
}

#[derive(Deserialize, TS)]
#[ts(
    export,
    export_to = "ui/api/models/auto-generates/DataSourceListParams.ts",
    rename = "DataSourceListParams"
)]
pub struct ListParams {
    pub database_name: Option<String>,
}

async fn find_by_id(
    state: State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> anyhow::Result<ResJson<Model>, AppError> {
    let res = DataSourceListService::find_by_id(&state.conn, id).await;
    data_response!(res)
}

async fn list(
    state: State<Arc<AppState>>,
    Json(payload): Json<PaginationPayload<ListParams>>,
) -> anyhow::Result<ResJsonWithPagination<Model>, AppError> {
    let res = DataSourceListService::list(
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
    let res = DataSourceListService::add(&state.conn, payload).await;

    data_response!(res)
}

async fn update_by_id(
    state: State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(payload): Json<Model>,
) -> anyhow::Result<ResJson<Model>, AppError> {
    let res = DataSourceListService::update_by_id(&state.conn, id, payload).await;

    data_response!(res)
}

async fn del(
    state: State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> anyhow::Result<ResJson<bool>, AppError> {
    let res = DataSourceListService::delete(&state.conn, id).await;

    bool_response!(res)
}

#[derive(Deserialize, TS)]
#[ts(
    export,
    export_to = "ui/api/models/auto-generates/QueryTableColumnsParameters.ts",
    rename = "QueryTableColumnsParameters"
)]
struct QueryTableColumnsParameters {
    #[ts(type = "any")]
    data_source: DataSource,
    table_name: String,
}

async fn query_table_columns(
    _: State<Arc<AppState>>,
    Json(payload): Json<QueryTableColumnsParameters>,
) -> anyhow::Result<ResJson<Vec<Value>>, AppError> {
    let res =
        DataSourceListService::query_table_columns(payload.data_source, payload.table_name).await;

    data_response!(res)
}
