use axum::{http::StatusCode, Json};
use anyhow::Result;
use axum::extract::{Path, State};
use serde::Deserialize;
use aide::{
    axum::{
        routing::{get, post},
        ApiRouter,
    },
};
use schemars::JsonSchema;

use crate::api::common::{ResTemplate, Pagination, AppState, ResJsonWithPagination, ResJson, Id};
use crate::entity::collect_config::Model;
use crate::service::collect_config_service::CollectConfigService;


pub fn set_routes() -> ApiRouter<AppState> {
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
    state: State<AppState>,
    Path(id): Path<Id>
) -> Result<ResJson<Model>, (StatusCode, String)> {
    let res = CollectConfigService::find_by_id(&state.conn, id.id)
        .await;

    data_response!(res)
}

async fn list(
    state: State<AppState>,
    Json(payload): Json<QueryList>
) -> Result<ResJsonWithPagination<Model>, (StatusCode, String)> {
    let res = CollectConfigService::list(&state.conn, payload.current, payload.page_size)
        .await;

    pagination_response!(res, payload.current, payload.page_size)

}
async fn add(
    state: State<AppState>,
    Json(payload): Json<Model>
) -> Result<ResJson<Model>, (StatusCode, String)> {
    let res = CollectConfigService::add(&state.conn, payload)
        .await;

    data_response!(res)
}

async fn update_by_id(
    state: State<AppState>,
    Path(id): Path<Id>,
    Json(payload): Json<Model>
) -> Result<ResJson<Model>, (StatusCode, String)>{
    let res = CollectConfigService::update_by_id(&state.conn, id.id, payload)
        .await;

    data_response!(res)
}

async fn delete(
    state: State<AppState>,
    Path(id): Path<Id>
) -> Result<ResJson<bool>, (StatusCode, String)>{
    let res = CollectConfigService::delete(&state.conn, id.id)
        .await;

    bool_response!(res)
}
