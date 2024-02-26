use std::sync::Arc;
use axum::extract::{Path, State};
use axum::{Json, Router};
use axum::routing::{get, post};
use serde::Deserialize;
use ts_rs::TS;
use crate::api::common::{AppError, AppState, PaginationPayload, ResJson, ResJsonWithPagination};
use crate::{bool_response, data_response, pagination_response};
use crate::entity::datasource_list::Model;
use crate::service::datasource_list_service::DataSourceListService;

pub fn set_routes() -> Router<Arc<AppState>> {
    let routes = Router::new()
        .route("/find_by_id/:id", get(find_by_id))
        .route("/list", post(list))
        .route("/add", post(add))
        .route("/update_by_id/:id", post(update_by_id))
        .route("/del/:id", get(del));

    routes
}

#[derive(Deserialize, TS)]
#[ts(
export,
export_to = "ui/api/models/auto-generates/DatasourceListParams.ts",
rename = "DatasourceListParams"
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