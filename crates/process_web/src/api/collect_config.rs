use anyhow::Result;
use axum::extract::{Path, State};
use axum::{Json};
use axum::{
    routing::{get, post},
    Router,
};
use process_core::http::HttpConfig;
use process_core::process::{Export, Receive, Serde};
use schemars::_serde_json::Value;
use std::sync::Arc;

use crate::api::common::{
    AppError, AppState, Pagination, ResJson, ResJsonWithPagination, ResTemplate, PaginationPayload
};
use crate::entity::collect_config::Model;
use crate::service::collect_config_service::CollectConfigService;

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
    let res = CollectConfigService::add(&state.conn, payload).await;

    data_response!(res)
}

async fn update_by_id(
    state: State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(payload): Json<Model>,
) -> Result<ResJson<Model>, AppError> {
    let res = CollectConfigService::update_by_id(&state.conn, id, payload).await;

    data_response!(res)
}

async fn del(
    state: State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<ResJson<bool>, AppError> {
    let res = CollectConfigService::delete(&state.conn, id).await;

    bool_response!(res)
}

/// 执行id所配置的采集任务
pub async fn execute(
    state: State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<ResJson<String>, AppError> {
    let data = CollectConfigService::find_by_id(&state.conn, id).await?;
    let mut http = process_core::http::Http::new();
    let mut headers = None;

    let get_map_rules = |value: Option<Value>| {
        // [["a", "b"]]
        if let Some(rules) = value {
            return rules
                .as_array()
                .unwrap()
                .iter()
                .map(|x| {
                    let temp = x.as_array().unwrap();
                    [
                        temp.get(0).unwrap().to_string(),
                        temp.get(1).unwrap().to_string(),
                    ]
                })
                .collect();
        }
        vec![]
    };

    if let Some(h) = data.headers {
        let temp = h
            .as_object()
            .unwrap()
            .iter()
            .map(|(key, value)| (key.clone(), value.to_string()))
            .collect::<Vec<(String, String)>>();
        headers = Some(temp)
    }

    let http_receive = http
        .receive(
            data.url,
            HttpConfig {
                method: data.method.parse().unwrap(),
                headers,
                body: data.body,
            },
        )
        .await?
        .add_map_rules(get_map_rules(data.map_rules))
        .serde()?
        .set_template_string(data.template_string)
        .export();

    data_response!(http_receive)
}
