use aide::axum::{
    routing::{get, post},
    ApiRouter,
};
use anyhow::Result;
use axum::extract::{Path, State};
use axum::{http::StatusCode, Json};
use process_core::http::HttpConfig;
use process_core::process::{Export, Receive, Serde};
use schemars::JsonSchema;
use schemars::_serde_json::Value;
use serde::Deserialize;
use std::sync::Arc;

use crate::api::common::{AppState, Id, Pagination, ResJson, ResJsonWithPagination, ResTemplate};
use crate::entity::collect_config::Model;
use crate::service::collect_config_service::CollectConfigService;

pub fn set_routes() -> ApiRouter<Arc<AppState>> {
    let routes = ApiRouter::new()
        .api_route("/find_by_id/:id", post(find_by_id))
        .api_route("/list", post(list))
        .api_route("/add", post(add))
        .api_route("/update_by_id/:id", post(update_by_id))
        .api_route("/delete/:id", get(delete))
        .api_route("/execute/:id", get(execute));

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
    let res = CollectConfigService::find_by_id(&state.conn, id.id).await;

    data_response!(res)
}

async fn list(
    state: State<Arc<AppState>>,
    Json(payload): Json<QueryList>,
) -> Result<ResJsonWithPagination<Model>, (StatusCode, String)> {
    let res = CollectConfigService::list(&state.conn, payload.current, payload.page_size).await;

    pagination_response!(res, payload.current, payload.page_size)
}
async fn add(
    state: State<Arc<AppState>>,
    Json(payload): Json<Model>,
) -> Result<ResJson<Model>, (StatusCode, String)> {
    let res = CollectConfigService::add(&state.conn, payload).await;

    data_response!(res)
}

async fn update_by_id(
    state: State<Arc<AppState>>,
    Path(id): Path<Id>,
    Json(payload): Json<Model>,
) -> Result<ResJson<Model>, (StatusCode, String)> {
    let res = CollectConfigService::update_by_id(&state.conn, id.id, payload).await;

    data_response!(res)
}

async fn delete(
    state: State<Arc<AppState>>,
    Path(id): Path<Id>,
) -> Result<ResJson<bool>, (StatusCode, String)> {
    let res = CollectConfigService::delete(&state.conn, id.id).await;

    bool_response!(res)
}

/// 执行id所配置的采集任务
async fn execute(
    state: State<Arc<AppState>>,
    Path(id): Path<Id>,
) -> Result<ResJson<bool>, (StatusCode, String)> {
    let data = CollectConfigService::find_by_id(&state.conn, id.id).await?;
    let mut http = process_core::http::Http::new();
    let mut headers = None;

    let get_map_rules = |value: Option<Value>| {
        // [["a", "b"]]
        if let Some(rules) = value {
            return rules
                .as_array()
                .ok_or("map_rules 无法解析")?
                .iter()
                .map(|x| {
                    let temp = x.as_array().unwrap();
                    vec![temp.get("0"), temp.get("1")]
                })
                .collect::<Vec<[String; 2]>>();
        }
        vec![]
    };

    if let Some(h) = data.headers {
        let temp = h
            .as_object()
            .ok_or("headers 无法解析")?
            .iter()
            .map(|item| item)
            .collect::<Vec<(String, String)>>();
        headers = Some(temp)
    }

    http.receive(
        data.url,
        HttpConfig {
            method: data.method.into(),
            headers,
            body: data.body,
        },
    )
    .await?
    .add_map_rules(get_map_rules(data.map_rules))
    .serde()?
    .set_template_string(data.template_string)
    .export()?;

    todo!();
    bool_response!(Ok(()))
}
