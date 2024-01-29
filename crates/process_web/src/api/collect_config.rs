use std::collections::HashMap;
use std::borrow::Borrow;
use anyhow::{anyhow, Result};
use axum::extract::{Path, State};
use axum::Json;
use axum::{
    routing::{get, post},
    Router,
};
use process_core::http::HttpConfig;
use process_core::process::{Export, Receive, Serde};
use schemars::_serde_json::Value;
use std::sync::Arc;
use serde_json::json;
use process_core::json::find_value;

use crate::api::common::{
    AppError, AppState, Pagination, PaginationPayload, ResJson, ResJsonWithPagination, ResTemplate,
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

async fn del(state: State<Arc<AppState>>, Path(id): Path<i32>) -> Result<ResJson<bool>, AppError> {
    let res = CollectConfigService::delete(&state.conn, id).await;

    bool_response!(res)
}

/// 执行id所配置的采集任务
pub async fn execute(
    state: State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<ResJson<Vec<String>>, AppError> {
    let data = CollectConfigService::find_by_id(&state.conn, id).await?;

    let res = process_data(&data).await;

    data_response!(res)
}

pub async fn process_data(data: &Model) -> Result<Vec<String>> {
    if let Some(loop_request_by_pagination) = data.loop_request_by_pagination {
        if loop_request_by_pagination {
            let mut sholud_stop = false;
            let current_key = data.current_key.as_ref().ok_or(anyhow!("请指定current_key"))?;
            let page_size_key = data.page_size_key.as_ref().ok_or(anyhow!("请指定page_size_key"))?;
            let max_number_of_result_data = data.max_number_of_result_data.ok_or(anyhow!("请指定max_number_of_result_data"))?;
            let max_count_of_request = data.max_count_of_request.ok_or(anyhow!("请指定max_count_of_request"))?;
            let body = serde_json::from_str::<Value>(data.body.clone().unwrap().as_str()).unwrap();

            let mut loop_counts = 0;

            let mut data_res = vec![];

            while !sholud_stop {
                let mut map = HashMap::new();

                for (key, value) in body.as_object().unwrap() {
                    if key == current_key {
                        let current = value.as_i64().unwrap() + loop_counts * body[&page_size_key].as_i64().unwrap();
                        map.insert(key ,json!(current));
                    } else {
                        map.insert(key, value.clone());
                    }
                }

                let (has_next_page, res) = process_data_req(&data, Some(json!(map).to_string())).await.unwrap();

                sholud_stop = !has_next_page;
                data_res = [data_res, res.unwrap()].concat();
                loop_counts += 1;

                if data_res.len() >= max_number_of_result_data as usize {
                    sholud_stop = true;
                }

                if loop_counts >= max_count_of_request as i64 {
                    sholud_stop = true;
                }

            }


            return Ok(data_res);

        }
    }

    let (_, res) = process_data_req(data, data.body.clone()).await.unwrap();
    res
}

pub async fn process_data_req(data: &Model, body: Option<String>) -> Result<(bool, Result<Vec<String>>)> {
    let mut http = process_core::http::Http::new();
    let mut headers = None;

    let get_map_rules = |value: Option<&Value>| {
        // [["a", "b"]]
        if let Some(rules) = value {
            return rules
                .as_array()
                .unwrap()
                .iter()
                .map(|x| {
                    [
                        x[0].as_str().unwrap().to_string(),
                        x[1].as_str().unwrap().to_string(),
                    ]
                })
                .collect();
        }
        vec![]
    };

    if let Some(h) = &data.headers {
        let temp = h
            .as_object()
            .unwrap()
            .iter()
            .map(|(key, value)| (key.clone(), value.as_str().unwrap().to_string()))
            .collect::<Vec<(String, String)>>();
        headers = Some(temp)
    }

    let mut http_receive = http
        .receive(
            data.url.clone(),
            HttpConfig {
                method: data.method.clone().parse().unwrap(),
                headers,
                body: body.clone(),
            },
        )
        .await?;

    let mut has_next_page = true;
    let filed_of_result_data = data.filed_of_result_data.as_ref().unwrap();


    if let Ok(found_data) = find_value(filed_of_result_data.borrow(), &http_receive.data) {
        if let Some(array) = found_data.as_array() {
            if array.is_empty() {
                has_next_page = false;
                return Ok((has_next_page, Ok(vec![])));
            }
        } else {
            has_next_page = false;
            return Ok((has_next_page, Ok(vec![])));
        }
    } else {
        has_next_page = false;
        return Ok((has_next_page, Ok(vec![])));
    }


    if data.map_rules.is_some() {
        if let Some(x) = &data.map_rules {
            if !x.as_array().unwrap().is_empty() {
                http_receive = http_receive.add_map_rules(get_map_rules(Some(x)))
                    .serde()?;
            }
        }
    }
    let res = http_receive.set_template_string(data.template_string.clone())
        .export();


    Ok((has_next_page, res))
}