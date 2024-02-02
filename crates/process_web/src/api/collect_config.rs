use std::borrow::Borrow;
use std::collections::HashMap;
use std::sync::Arc;

use anyhow::{anyhow, Result};
use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use process_core::http::HttpConfig;
use process_core::json::find_value;
use process_core::process::{Export, Receive, Serde};
use schemars::_serde_json::Value;
use tracing::{debug, error};

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
    // TODO 创建一条采集任务，然后调度执行。
    let data = CollectConfigService::find_by_id(&state.conn, id).await?;
    tokio::spawn(async move {
        let res = process_data(&data).await;
        match res {
            Ok(list) => {
                match CollectConfigService::cache_data(&state.cache_conn, &list).await {
                    Ok(_) => {}
                    Err(err) => error!("{}", err),
                };
            }
            Err(err) => {
                error!("{}", err.to_string());
            }
        }
    });
    // https://docs.rs/tokio/1.35.1/tokio/task/index.html#yield_now
    tokio::task::yield_now().await;

    Ok(Json(res_template_ok!(Some(true))))
}

pub async fn process_data(data: &Model) -> Result<Vec<String>> {
    let body_string = format_body_string(data.body.as_ref());

    if let Some(loop_request_by_pagination) = data.loop_request_by_pagination {
        if loop_request_by_pagination {
            let mut sholud_stop = false;
            let max_number_of_result_data = data
                .max_number_of_result_data
                .ok_or(anyhow!("请指定max_number_of_result_data"))?;
            let max_count_of_request = data
                .max_count_of_request
                .ok_or(anyhow!("请指定max_count_of_request"))?;
            let body =
                serde_json::from_str::<Value>(body_string.clone().unwrap().as_str()).unwrap();

            let mut loop_counts = 0;

            let mut data_res = vec![];

            debug!("开始进行分页请求，max_number_of_result_data: {max_number_of_result_data}, max_count_of_request: {max_count_of_request}");
            while !sholud_stop {
                let mut body_string = body_string.clone().unwrap_or_default();
                let mut new_string = body_string.as_str();
                let mut map_str = HashMap::new();

                while let Some(l_i) = new_string.find("${") {
                    new_string = &new_string[l_i..];

                    if let Some(r_i) = new_string.find("}") {
                        let current_str = &new_string[..r_i + 1];
                        let mut parmater_str = new_string[2..r_i].to_string();

                        // 对表达式中的值进行计算_loop_counts
                        if parmater_str.contains("_loop_counts") {
                            parmater_str = parmater_str
                                .replace("_loop_counts", loop_counts.to_string().as_str());

                            for (key, value) in body.as_object().unwrap() {
                                if parmater_str.contains(key) {
                                    parmater_str =
                                        parmater_str.replace(key, value.to_string().as_str());
                                }
                            }
                            parmater_str = math_parse::MathParse::parse(parmater_str.as_str())
                                .unwrap()
                                .solve_float(None)
                                .unwrap_or(0.0)
                                .to_string();
                        }
                        map_str.insert(parmater_str, current_str.to_string());
                        new_string = &new_string[r_i..];
                    }
                }
                for (value, value2) in map_str {
                    body_string = body_string.replace(value2.as_str(), &value);
                }

                let (has_next_page, res) =
                    process_data_req(&data, Some(body_string.to_string())).await?;
                let new_vec = res?;

                sholud_stop = !has_next_page;
                data_res = [data_res, new_vec].concat();
                loop_counts += 1;

                if data_res.len() >= max_number_of_result_data as usize {
                    sholud_stop = true;
                }

                if loop_counts >= max_count_of_request as i64 {
                    sholud_stop = true;
                }
            }
            debug!("分页请求结束, {:?}", data_res);

            return Ok(data_res);
        }
    }

    let (_, res) = process_data_req(data, body_string.clone()).await.unwrap();

    res
}

/// 查找body字符串中`${xxx}`格式值进行转换
/// 目前只支持日期字符串
pub fn format_body_string(body: Option<&String>) -> Option<String> {
    if body.is_none() {
        return None;
    }

    let mut body_str = body.unwrap().as_str();

    let mut value_map = HashMap::new();

    while let Some(i) = body_str.find("${") {
        body_str = &body_str[i..];

        if let Some(j) = body_str.find("}") {
            let params_str = &body_str[2..j];
            let current_str = &body_str[0..j + 1];
            if params_str.contains("_now") {
                let date = get_datetime_by_string(params_str).unwrap_or("".to_string());
                value_map.insert(date, current_str);
            }
            //Tips 含_loop_counts的需要在循环请求中去获取参数，不在此处做处理

            body_str = &body_str[j..];
        } else {
            break;
        }
    }

    let mut new_str = body.unwrap().clone();

    for (key, value) in value_map {
        new_str = new_str.replace(value, key.as_str());
    }

    Some(new_str)
}

pub async fn process_data_req(
    data: &Model,
    body: Option<String>,
) -> Result<(bool, Result<Vec<String>>)> {
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
                body,
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
                http_receive = http_receive.add_map_rules(get_map_rules(Some(x))).serde()?;
            }
        }
    }

    let res = http_receive
        .set_template_string(data.template_string.clone())
        .export();

    Ok((has_next_page, res))
}

pub fn get_date(str: &str) -> Result<chrono::Duration> {
    let number = str[..str.len() - 1].parse::<i64>()?;
    if str.contains("d") {
        return Ok(chrono::Duration::days(number));
    }
    if str.contains("h") {
        return Ok(chrono::Duration::hours(number));
    }
    if str.contains("m") {
        return Ok(chrono::Duration::minutes(number));
    }
    if str.contains("s") {
        return Ok(chrono::Duration::seconds(number));
    }
    Err(anyhow!("未发现匹配的字符"))
}

/// 根据特定字符串获取当地时间日期，支持加减法计算
/// 例如："now+1d-24h-60m+60s.%Y-%m-%d %H:%M:%S"
///
///  1. now必须指定
///  2. 1d表示1天，1h表示1小时，1m表示1分钟，1s表示1秒
///  3. `.`前面是日期计算字符串，`.`后面是格式化字符串，参考：`<https://docs.rs/chrono/latest/chrono/format/strftime/index.html>`
///
///     `%Y-%m-%d %H:%M:%S`输出"2024-01-31 15:12:48"格式的日期
///
/// ```
///     use process_web::api::collect_config::*;
///
///     let time_strimg = get_datetime_by_string(&r#"now+1d-24h-60m+60s.%Y-%m-%d %H:%M:%S"#.to_string());
///
///     println!("time_strimg {:?}", time_strimg);
/// ```
pub fn get_datetime_by_string(value_str: &str) -> Result<String> {
    let split_str: Vec<&str> = value_str.split('.').collect();

    if split_str.len() > 0 {
        let date_str = split_str[0];

        if !date_str.contains("now") {
            return Err(anyhow!("请指定now"));
        }

        let mut date = chrono::Local::now();
        let mut last_i = 0;
        let mut pre_str = "";
        let mut current_sign = "";

        for i in 0..date_str.len() {
            let char = &date_str[i..i + 1];
            if char == "-" || char == "+" {
                if pre_str == "" {
                    pre_str = &date_str[last_i..i];
                    current_sign = char;
                    last_i = i + 1;
                    continue;
                }

                pre_str = &date_str[last_i..i];
                if current_sign == "-" {
                    date = date - get_date(pre_str)?;
                } else if current_sign == "+" {
                    date = date + get_date(pre_str)?;
                }

                last_i = i + 1;
                current_sign = char;
            }
        }

        pre_str = &date_str[last_i..];
        if current_sign == "-" {
            date = date - get_date(pre_str)?;
        } else if current_sign == "+" {
            date = date + get_date(pre_str)?;
        }

        if split_str.len() > 1 {
            let format_str = split_str[1];
            return Ok(date.naive_local().format(format_str).to_string());
        }
        return Ok(date.naive_local().to_string());
    }

    Err(anyhow!("无法解析字符串"))
}
