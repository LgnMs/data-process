use std::borrow::Borrow;
use std::collections::HashMap;
use std::sync::Arc;

use crate::api::collect_config::ListParams;
use anyhow::anyhow;
use chrono::Local;
use process_core::http::HttpConfig;
use process_core::json::find_value;
use process_core::process::{Export, Receive, Serde};
use sea_orm::ActiveValue::{Set, Unchanged};
use sea_orm::*;
use tokio_cron_scheduler::{Job, JobSchedulerError};
use tracing::{debug, error, warn};
use uuid::Uuid;

use crate::api::common::AppState;
use crate::entity::collect_config::Model;
use crate::entity::{collect_config, collect_log};
use crate::service::collect_log_service::CollectLogService;
use crate::utils::{format_body_string, format_cron, job_err_to_db_err};

pub struct CollectConfigService;

impl CollectConfigService {
    pub async fn find_by_id(db: &DbConn, id: i32) -> Result<Model, DbErr> {
        collect_config::Entity::find_by_id(id)
            .filter(collect_config::Column::DelFlag.eq(0))
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find data by id.".to_owned()))
    }

    pub async fn list(
        db: &DbConn,
        page: u64,
        page_size: u64,
        data: Option<ListParams>,
    ) -> Result<(Vec<Model>, u64), DbErr> {
        let mut conditions = Condition::all();
        if let Some(data) = data {
            if let Some(name) = data.name {
                conditions = conditions.add(collect_config::Column::Name.contains(&name));
            }
        }

        let paginator = collect_config::Entity::find()
            .filter(collect_config::Column::DelFlag.eq(0))
            .filter(conditions)
            .order_by_desc(collect_config::Column::Id)
            .paginate(db, page_size);

        let num_pages = paginator.num_items().await?;

        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }

    pub async fn add(state: Arc<AppState>, data: Model) -> Result<Model, DbErr> {
        Self::save(state, None, data).await
    }

    pub async fn update_by_id(state: Arc<AppState>, id: i32, data: Model) -> Result<Model, DbErr> {
        Self::save(state, Some(id), data).await
    }

    pub async fn update_job_id_by_id(
        state: Arc<AppState>,
        job_id: Option<Uuid>,
        id: i32,
    ) -> Result<Model, DbErr> {
        let mut db_data = collect_config::Entity::find_by_id(id)
            .one(&state.conn)
            .await?
            .ok_or(DbErr::Custom("Cannot find data by id.".to_owned()))?
            .into_active_model();

        db_data.job_id = Set(job_id);

        db_data.update(&state.conn).await
    }

    pub async fn save(state: Arc<AppState>, id: Option<i32>, data: Model) -> Result<Model, DbErr> {
        debug!("data: {:?}, id: {:?}", data, id);
        let now = Local::now().naive_local();

        let data_clone = data.clone();
        let mut active_data = collect_config::ActiveModel {
            url: Set(data_clone.url),
            name: Set(data_clone.name),
            desc: Set(data_clone.desc),
            method: Set(data_clone.method),
            headers: Set(data_clone.headers),
            body: Set(data_clone.body),
            map_rules: Set(data_clone.map_rules),
            template_string: Set(data_clone.template_string),
            loop_request_by_pagination: Set(data_clone.loop_request_by_pagination),
            cache_table_name: Set(data_clone.cache_table_name),
            max_number_of_result_data: Set(data_clone.max_number_of_result_data),
            filed_of_result_data: Set(data_clone.filed_of_result_data),
            max_count_of_request: Set(data_clone.max_count_of_request),
            cron: Set(data_clone.cron),
            db_columns_config: Set(data_clone.db_columns_config),
            ..Default::default()
        };

        if let Some(id) = id {
            // 更新
            let db_data = collect_config::Entity::find_by_id(id)
                .one(&state.conn)
                .await?
                .ok_or(DbErr::Custom("Cannot find data by id.".to_owned()))?;

            active_data.id = Unchanged(db_data.id);
            active_data.update_time = Set(now);

            if let Some(db_columns_config) = data.db_columns_config.as_ref() {
                if db_data.db_columns_config != data.db_columns_config {
                    Self::update_table_struct(
                        &state.cache_conn,
                        Some(db_columns_config),
                        data.cache_table_name.as_ref().unwrap(),
                    )
                    .await?;
                }
            }
            if db_data.cache_table_name != data.cache_table_name {
                Self::update_table_struct(
                    &state.cache_conn,
                    None,
                    data.cache_table_name.as_ref().unwrap(),
                )
                .await?;
            }

            let job_id = update_job_scheduler(state.clone(), &data, &db_data)
                .await
                .map_err(job_err_to_db_err)?;
            if let Some(new_job_id) = job_id {
                active_data.job_id = Set(Some(new_job_id));
            }

            active_data.update(&state.conn).await
        } else {
            active_data.create_time = Set(now);
            active_data.update_time = Set(now);
            if let Some(db_columns_config) = data.db_columns_config.as_ref() {
                Self::create_table(
                    &state.cache_conn,
                    db_columns_config,
                    data.cache_table_name.as_ref().unwrap(),
                )
                .await?;
            }
            let job_id = create_job_scheduler(state.clone(), &data)
                .await
                .map_err(job_err_to_db_err)?;
            active_data.job_id = Set(job_id);

            active_data.insert(&state.conn).await
        }
    }

    pub async fn delete(state: Arc<AppState>, id: i32) -> Result<Model, DbErr> {
        let data = collect_config::Entity::find_by_id(id)
            .one(&state.conn)
            .await?
            .ok_or(DbErr::Custom("Cannot find data by id.".to_owned()))?;

        if let Some(job_id) = data.job_id {
            state
                .sched
                .remove(&job_id)
                .await
                .map_err(job_err_to_db_err)?;
        }
        let mut active_data = data.into_active_model();

        active_data.del_flag = Set(1);
        active_data.job_id = Set(None);

        active_data.update(&state.conn).await
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
        db_columns_config: Option<&serde_json::Value>,
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

        if let Some(x) = db_columns_config {
            let res = Self::create_table(cache_db, x, table_name).await?;
            return Ok(res);
        }
        Ok(true)
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

    /// 初始化所有的采集系统调度任务
    pub async fn setup_collect_config_cron(state: &Arc<AppState>) -> anyhow::Result<()> {
        let list = collect_config::Entity::find()
            .filter(collect_config::Column::DelFlag.eq(0))
            .all(&state.conn)
            .await?;

        for item in list {
            let state = state.clone();
            let item = item.clone();
            let job_id = create_job_scheduler(state.clone(), &item).await?;

            CollectConfigService::update_job_id_by_id(state, job_id, item.id).await?;
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
                    collect_data_with_http(&data, Some(body_string.to_string())).await?;
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

    let (_, res) = collect_data_with_http(data, body_string.clone())
        .await
        .unwrap();

    res
}

pub async fn collect_data_with_http(
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

    if let Some(filed_of_result_data) = data.filed_of_result_data.as_ref() {
        if let Some(found_data) = find_value(filed_of_result_data.borrow(), &http_receive.data) {
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
    } else {
        has_next_page = false;
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

async fn update_job_scheduler(
    state: Arc<AppState>,
    data: &Model,
    db_data: &Model,
) -> Result<Option<Uuid>, JobSchedulerError> {
    if db_data.cron != data.cron {
        if let Some(job_id) = db_data.job_id {
            state.sched.remove(&job_id).await?;
        }
        return create_job_scheduler(state, data).await;
    }
    Ok(None)
}

async fn create_job_scheduler(
    state: Arc<AppState>,
    data: &Model,
) -> Result<Option<Uuid>, JobSchedulerError> {
    if let Some(cron) = data.cron.as_ref() {
        let sched_state = state.clone();
        let data = data.clone();
        let job_id = state
            .sched
            .add(Job::new_async(
                format_cron(cron.clone()).as_str(),
                move |_uuid, mut _l| {
                    let st = sched_state.clone();
                    let item_c = data.clone();
                    Box::pin(async move {
                        CollectConfigService::execute_task(&st, &item_c).await;
                    })
                },
            )?)
            .await?;
        return Ok(Some(job_id));
    } else {
        let err = format!(
            "采集配置：{} 定时任务添加失败 cron: {:?} ",
            data.name, data.cron
        );
        warn!("{}", err);
    }
    Ok(None)
}
