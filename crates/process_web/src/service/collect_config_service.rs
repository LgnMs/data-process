use std::borrow::Borrow;
use std::collections::HashMap;
use std::sync::Arc;

use anyhow::anyhow;
use chrono::Local;
use process_core::http::HttpConfig;
use process_core::json::find_value;
use process_core::process::{Export, Receive, Serde};
use sea_orm::ActiveValue::{Set, Unchanged};
use sea_orm::*;
use tokio_cron_scheduler::Job;
use tracing::{debug, error, warn};
use uuid::Uuid;

use crate::api::common::AppState;
use crate::entity::collect_config::Model;
use crate::entity::{collect_config, collect_log};
use crate::service::collect_log_service::CollectLogService;

pub struct CollectConfigService;

impl CollectConfigService {
    pub async fn find_by_id(db: &DbConn, id: i32) -> Result<Model, DbErr> {
        collect_config::Entity::find_by_id(id)
            .filter(collect_config::Column::DelFlag.eq(0))
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find data by id.".to_owned()))
    }

    pub async fn list(db: &DbConn, page: u64, page_size: u64) -> Result<(Vec<Model>, u64), DbErr> {
        let paginator = collect_config::Entity::find()
            .filter(collect_config::Column::DelFlag.eq(0))
            .order_by_desc(collect_config::Column::Id)
            .paginate(db, page_size);

        let num_pages = paginator.num_items().await?;

        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }

    pub async fn add(db: &DbConn, cache_db: &DbConn, data: Model) -> Result<Model, DbErr> {
        Self::save(db, cache_db, None, data).await
    }

    pub async fn update_by_id(
        db: &DbConn,
        cache_db: &DbConn,
        id: i32,
        data: Model,
    ) -> Result<Model, DbErr> {
        Self::save(db, cache_db, Some(id), data).await
    }

    pub async fn save(
        db: &DbConn,
        cache_db: &DbConn,
        id: Option<i32>,
        data: Model,
    ) -> Result<Model, DbErr> {
        debug!("data: {:?}, id: {:?}", data, id);
        let now = Local::now().naive_local();

        let mut active_data = collect_config::ActiveModel {
            url: Set(data.url),
            name: Set(data.name),
            desc: Set(data.desc),
            method: Set(data.method),
            headers: Set(data.headers),
            body: Set(data.body),
            map_rules: Set(data.map_rules),
            template_string: Set(data.template_string),
            loop_request_by_pagination: Set(data.loop_request_by_pagination),
            cache_table_name: Set(data.cache_table_name.clone()),
            max_number_of_result_data: Set(data.max_number_of_result_data),
            filed_of_result_data: Set(data.filed_of_result_data),
            max_count_of_request: Set(data.max_count_of_request),
            cron: Set(data.cron),
            db_columns_config: Set(data.db_columns_config.clone()),
            ..Default::default()
        };
        if let Some(id) = id {
            let db_data = collect_config::Entity::find_by_id(id)
                .one(db)
                .await?
                .ok_or(DbErr::Custom("Cannot find data by id.".to_owned()))?;

            active_data.id = Unchanged(db_data.id);
            active_data.update_time = Set(now);

            if let Some(db_columns_config) = data.db_columns_config.as_ref() {
                if db_data.db_columns_config != data.db_columns_config
                    || db_data.cache_table_name != data.cache_table_name
                {
                    Self::update_table_struct(
                        cache_db,
                        db_columns_config,
                        data.cache_table_name.as_ref().unwrap(),
                    )
                    .await?;
                }
            }

            active_data.update(db).await
        } else {
            active_data.create_time = Set(now);
            active_data.update_time = Set(now);
            if let Some(db_columns_config) = data.db_columns_config.as_ref() {
                Self::create_table(
                    cache_db,
                    db_columns_config,
                    data.cache_table_name.as_ref().unwrap(),
                )
                .await?;
            }
            active_data.insert(db).await
        }
    }

    pub async fn delete(db: &DbConn, id: i32) -> Result<Model, DbErr> {
        let mut collect_config: collect_config::ActiveModel =
            collect_config::Entity::find_by_id(id)
                .one(db)
                .await?
                .ok_or(DbErr::Custom("Cannot find data by id.".to_owned()))
                .map(Into::into)?;

        collect_config.del_flag = Set(1);

        collect_config.update(db).await
    }

    pub async fn cache_data(cache_db: &DbConn, list: &Vec<String>) -> Result<bool, DbErr> {
        for item in list {
            cache_db
                .execute(Statement::from_string(
                    cache_db.get_database_backend(),
                    item,
                ))
                .await?;
        }
        Ok(true)
    }

    pub async fn create_table(
        cache_db: &DbConn,
        db_columns_config: &serde_json::Value,
        table_name: &String,
    ) -> Result<bool, DbErr> {
        if let Some(db_columns_config) = db_columns_config.as_array() {
            // TODO 适配MYSQL
            let mut template_str = format!("CREATE TABLE IF NOT EXISTS {table_name}");
            let mut column_str = Vec::with_capacity(db_columns_config.len() + 1);
            let mut have_id_key = false;

            for item in db_columns_config {
                if item["key"] == "id" {
                    column_str.insert(0, format!("{} {} NOT NULL", item["key"], item["type"]));
                    have_id_key = true;
                } else {
                    column_str.push(format!("{} {} NULL", item["key"], item["type"]));
                }
            }

            if !have_id_key {
                column_str.insert(0, r#"id serial NOT NULL"#.to_string());
            }
            column_str.push(format!(
                "CONSTRAINT {table_name}_{:?}_pk PRIMARY KEY (id)",
                Local::now().naive_local().timestamp()
            ));
            template_str = format!("{} ({});", template_str, column_str.join(", "));

            cache_db
                .execute(Statement::from_string(
                    cache_db.get_database_backend(),
                    template_str,
                ))
                .await?;
        } else {
            return Err(DbErr::Custom(
                "db_columns_config无法解析为json数组".to_owned(),
            ));
        }

        Ok(true)
    }
    pub async fn update_table_struct(
        cache_db: &DbConn,
        db_columns_config: &serde_json::Value,
        table_name: &String,
    ) -> Result<bool, DbErr> {
        let now = Local::now().naive_utc().timestamp();
        let alert_sql = format!("ALTER TABLE {table_name} rename to __{table_name}_{now}");
        match cache_db
            .execute(Statement::from_string(
                cache_db.get_database_backend(),
                alert_sql,
            ))
            .await
        {
            Err(err) => {
                // TODO 识别为表不存在的错误
                error!("DbErr {:?}", err);
            }
            _ => {}
        };

        Self::create_table(cache_db, db_columns_config, table_name).await
    }

    pub async fn execute_task(state: &Arc<AppState>, data: &Model) {
        let collect_log_id = Uuid::new_v4();
        let collect_log_model = collect_log::Model {
            id: collect_log_id,
            collect_config_id: Some(data.id),
            status: 0,
            running_log: Some(String::new()),
            ..Default::default()
        };

        if let Some(err) = CollectLogService::add(&state.conn, collect_log_model)
            .await
            .err()
        {
            error!("任务日志添加失败 {err}");
        };

        let mut status = 1;
        let model = collect_log::Model {
            status,
            running_log: Some("开始执行采集任务!".to_string()),
            ..Default::default()
        };
        if let Some(err) = CollectLogService::update_by_id(&state.conn, collect_log_id, model)
            .await
            .err()
        {
            error!("status: {status} 运行中；日志更新失败: {err}");
        };

        let mut collect_log_string = String::new();

        collect_log_string.push_str(format!("采集配置： {:?}\n", data).as_str());
        let res = process_data(&data).await;
        match res {
            Ok(list) => {
                collect_log_string.push_str(format!("成功生成SQL： {:?}\n", list).as_str());
                match Self::cache_data(&state.cache_conn, &list).await {
                    Ok(_) => {
                        collect_log_string.push_str("采集任务执行成功!\n");
                        status = 2;
                    }
                    Err(err) => {
                        let err_str = format!("{}\n", err);
                        collect_log_string.push_str(err_str.as_str());
                        status = 3;
                        error!("status: {status} 运行失败；日志更新失败: {err_str}");
                    }
                };
            }
            Err(err) => {
                let err_str = format!("{}\n", err);
                collect_log_string.push_str(err_str.as_str());
                status = 3;
                error!("status: {status} 运行失败；日志更新失败: {err_str}");
            }
        }

        let model = collect_log::Model {
            status,
            running_log: Some(collect_log_string),
            ..Default::default()
        };
        if let Some(err) = CollectLogService::update_by_id(&state.conn, collect_log_id, model)
            .await
            .err()
        {
            error!("status: {status} 运行完毕；日志更新失败: {err}");
        };
    }

    pub async fn setup_collect_config_cron(
        state: &Arc<AppState>,
    ) -> anyhow::Result<()> {
        let list = collect_config::Entity::find().all(&state.conn).await?;

        for item in list {
            let state = state.clone();
            let item_c = item.clone();

            match item.cron {
                None => {
                    let err = format!(
                        "采集配置：{} 定时任务添加失败 cron: {:?} ",
                        item.name, item.cron
                    );
                    warn!("{}", err);
                }
                Some(cron) => {
                    // FIXME 匹配cron的格式
                    state.clone().sched
                        .add(Job::new_async(cron.as_str(), move |_uuid, mut _l| {
                            let st = state.clone();
                            let item_c = item_c.clone();
                            Box::pin(async move {
                                Self::execute_task(&st, &item_c).await;
                            })
                        })?)
                        .await?;
                }
            }
        }
        state.sched.start().await?;

        Ok(())
    }
}

pub async fn process_data(data: &Model) -> anyhow::Result<Vec<String>> {
    let body_string = format_body_string(data.body.as_ref());

    if let Some(loop_request_by_pagination) = data.loop_request_by_pagination {
        if loop_request_by_pagination {
            let mut should_stop = false;
            let max_number_of_result_data = data
                .max_number_of_result_data
                .ok_or(anyhow!("请指定max_number_of_result_data"))?;
            let max_count_of_request = data
                .max_count_of_request
                .ok_or(anyhow!("请指定max_count_of_request"))?;
            let body =
                serde_json::from_str::<serde_json::Value>(body_string.clone().unwrap().as_str())
                    .unwrap();

            let mut loop_counts = 0;

            let mut data_res = vec![];

            debug!("开始进行分页请求，max_number_of_result_data: {max_number_of_result_data}, max_count_of_request: {max_count_of_request}");
            while !should_stop {
                let mut body_string = body_string.clone().unwrap_or_default();
                let mut new_string = body_string.as_str();
                let mut map_str = HashMap::new();

                while let Some(l_i) = new_string.find("${") {
                    new_string = &new_string[l_i..];

                    if let Some(r_i) = new_string.find("}") {
                        let current_str = &new_string[..r_i + 1];
                        let mut parameter_str = new_string[2..r_i].to_string();

                        // 对表达式中的值进行计算_loop_counts
                        if parameter_str.contains("_loop_counts") {
                            parameter_str = parameter_str
                                .replace("_loop_counts", loop_counts.to_string().as_str());

                            for (key, value) in body.as_object().unwrap() {
                                if parameter_str.contains(key) {
                                    parameter_str =
                                        parameter_str.replace(key, value.to_string().as_str());
                                }
                            }
                            parameter_str = math_parse::MathParse::parse(parameter_str.as_str())
                                .unwrap()
                                .solve_float(None)
                                .unwrap_or(0.0)
                                .to_string();
                        }
                        map_str.insert(parameter_str, current_str.to_string());
                        new_string = &new_string[r_i..];
                    }
                }
                for (value, value2) in map_str {
                    body_string = body_string.replace(value2.as_str(), &value);
                }

                let (has_next_page, res) =
                    process_data_req(&data, Some(body_string.to_string())).await?;
                let new_vec = res?;

                should_stop = !has_next_page;
                data_res = [data_res, new_vec].concat();
                loop_counts += 1;

                if data_res.len() >= max_number_of_result_data as usize {
                    should_stop = true;
                }

                if loop_counts >= max_count_of_request as i64 {
                    should_stop = true;
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
) -> anyhow::Result<(bool, anyhow::Result<Vec<String>>)> {
    let mut http = process_core::http::Http::new();
    let mut headers = None;

    let get_map_rules = |value: Option<&serde_json::Value>| {
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

pub fn get_date(str: &str) -> anyhow::Result<chrono::Duration> {
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
///     use process_web::service::collect_config_service::*;
///
///     let time_string = get_datetime_by_string(&r#"now+1d-24h-60m+60s.%Y-%m-%d %H:%M:%S"#.to_string());
///
///     println!("time_string {:?}", time_string);
/// ```
pub fn get_datetime_by_string(value_str: &str) -> anyhow::Result<String> {
    let split_str: Vec<&str> = value_str.split('.').collect();

    if split_str.len() > 0 {
        let date_str = split_str[0];

        if !date_str.contains("now") {
            return Err(anyhow!("请指定now"));
        }

        let mut date = Local::now();
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
