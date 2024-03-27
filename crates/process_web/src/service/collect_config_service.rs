use std::borrow::Borrow;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use crate::api::collect_config::ListParams;
use anyhow::anyhow;
use chrono::Local;
use process_core::http::{HttpConfig, NestedConfig};
use process_core::json::find_value;
use process_core::process::{Export, Receive, Serde};
use sea_orm::ActiveValue::{Set, Unchanged};
use sea_orm::*;

use tokio_cron_scheduler::{Job, JobSchedulerError};
use tracing::{debug, error, warn};
use uuid::Uuid;

use crate::api::common::{pg_to_mysql_type, AppState};
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
            nested_config: Set(data_clone.nested_config),
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

            let mut data = data.clone();
            data.id = id;
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

    pub async fn cache_data(cache_db: &DbConn, list: &Vec<String>) -> Result<(), String> {
        let mut err_msg = String::new();
        for i in 0..list.len() {
            let item = &list[i];
            match cache_db
                .execute(Statement::from_string(
                    cache_db.get_database_backend(),
                    item,
                ))
                .await
            {
                Ok(msg) => {
                    println!("{:?}", msg);
                }
                Err(err) => {
                    err_msg.push_str(
                        format!("第{}条SQL执行失败，{}", i + 1, err.to_string()).as_str(),
                    );
                    err_msg.push_str("\n");
                }
            }
        }

        if err_msg.is_empty() {
            Ok(())
        } else {
            Err(err_msg)
        }
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

            match cache_db.get_database_backend() {
                DatabaseBackend::Postgres => {
                    for item in db_columns_config {
                        if item["key"] == "id" {
                            column_str
                                .insert(0, format!("{} {} NOT NULL", item["key"], item["type"]));
                            have_id_key = true;
                        } else {
                            column_str.push(format!("{} {} NULL", item["key"], item["type"]));
                        }
                    }
                }
                DatabaseBackend::MySql => {
                    for item in db_columns_config {
                        if item["key"] == "id" {
                            column_str.insert(
                                0,
                                format!(
                                    "`{}` {} NOT NULL",
                                    item["key"].as_str().unwrap(),
                                    pg_to_mysql_type(item["type"].as_str().unwrap()).unwrap()
                                ),
                            );
                            have_id_key = true;
                        } else {
                            column_str.push(format!(
                                "`{}` {} NULL",
                                item["key"].as_str().unwrap(),
                                pg_to_mysql_type(item["type"].as_str().unwrap()).unwrap()
                            ));
                        }
                    }
                }
                _ => {
                    error!("不支持的数据库格式");
                }
            }

            if !have_id_key {
                match cache_db.get_database_backend() {
                    DatabaseBackend::Postgres => {
                        column_str.insert(0, r#"id serial NOT NULL"#.to_string());
                        column_str.push(format!(
                            "CONSTRAINT {table_name}_{:?}_pk PRIMARY KEY (id)",
                            Local::now().naive_local().timestamp()
                        ));
                    }
                    DatabaseBackend::MySql => {
                        column_str.insert(0, r#"id INT AUTO_INCREMENT NOT NULL"#.to_string());
                        column_str.insert(0, r#"PRIMARY KEY (id)"#.to_string());
                    }
                    _ => {
                        error!("不支持的数据库格式");
                    }
                }
            }
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
        let mut collect_log_model = collect_log::Model {
            collect_config_id: Some(data.id),
            status: 0,
            running_log: String::new(),
            ..Default::default()
        };

        match CollectLogService::add(&state.conn, collect_log_model.clone()).await {
            Ok(db_log_data) => {
                collect_log_model = db_log_data;
            }
            Err(err) => {
                error!("任务日志添加失败 {err}");
            }
        }
        let log_id = collect_log_model.id;

        let _ = process_data(&data, state, log_id).await;
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

            Self::update_job_id_by_id(state, job_id, item.id).await?;
        }

        Ok(())
    }
}

pub async fn process_data(
    data: &Model,
    state: &Arc<AppState>,
    log_id: i32,
) -> anyhow::Result<Vec<String>> {
    let body_string = format_body_string(data.body.as_ref());

    if let Some(err) = CollectLogService::update_by_id(
        &state.conn,
        log_id,
        collect_log::Model {
            status: 1,
            running_log: format!("开始执行采集任务!\n采集配置： {:?}\n", data),
            ..Default::default()
        },
    )
        .await
        .err()
    {
        error!("status: {status} 运行完毕；日志更新失败: {err}");
    };

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
            let mut re_request_times = 0;
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

                match collect_data_with_http(&data, Some(body_string.to_string())).await {
                    Ok((has_next_page, res)) => {
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

                        if should_stop {
                            let mut res_data_str = String::new();
                            let mut collect_log_string = String::new();
                            if let Some(str) = data_res.get(0) {
                                res_data_str.push_str(str);
                                res_data_str.push_str("......");
                            } else {
                                res_data_str.push_str("空，请检查接口返回的数据与配置中的映射关系")
                            }

                            let status;
                            match CollectConfigService::cache_data(&state.cache_conn, &data_res)
                                .await
                            {
                                Ok(_) => {
                                    let log = format!("已累计发起{loop_counts}次请求，本轮采集{}条数据开始插入!\n 处理后的数据为", data_res.len());
                                    collect_log_string.push_str(log.as_str());
                                    collect_log_string.push_str(res_data_str.as_str());
                                    status = 2;
                                }
                                Err(err) => {
                                    let log = format!("已累计发起{loop_counts}次请求，本轮采集{}条数据开始插入!\n 处理后的数据为", data_res.len());
                                    collect_log_string.push_str(log.as_str());
                                    collect_log_string.push_str(res_data_str.as_str());
                                    collect_log_string.push_str("\n");
                                    collect_log_string.push_str(err.as_str());
                                    status = 3;
                                }
                            };

                            if let Some(err) = CollectLogService::update_by_id(
                                &state.conn,
                                log_id,
                                collect_log::Model {
                                    status,
                                    running_log: collect_log_string,
                                    ..Default::default()
                                },
                            )
                            .await
                            .err()
                            {
                                error!("status: 3 运行完毕；日志更新失败: {err}");
                            };
                        }
                    }
                    Err(err) => {
                        let log = anyhow!("循环请求因为异常中断,将在3s后再次尝试发起请求,重新请求次数为{re_request_times}次,上限为10次 {}", err);
                        debug!("{}", log);
                        if let Some(err) = CollectLogService::update_by_id(
                            &state.conn,
                            log_id,
                            collect_log::Model {
                                status: 1,
                                running_log: log.to_string(),
                                ..Default::default()
                            },
                        )
                        .await
                        .err()
                        {
                            error!("status: 3 运行完毕；日志更新失败: {err}");
                        };
                        if re_request_times < 10 {
                            re_request_times += 1;
                            should_stop = false;
                            tokio::time::sleep(Duration::from_secs(3)).await;
                        } else {
                            should_stop = true;
                            let log = anyhow!("多次请求后依然失败 {} ", err);
                            debug!("{}", log);
                            if let Some(err) = CollectLogService::update_by_id(
                                &state.conn,
                                log_id,
                                collect_log::Model {
                                    status: 3,
                                    running_log: log.to_string(),
                                    ..Default::default()
                                },
                            )
                            .await
                            .err()
                            {
                                error!("status: 3 运行完毕；日志更新失败: {err}");
                            };
                        }
                    }
                }

                // 如果数据大于10000条就开始入库
                if data_res.len() > 10000 || (loop_counts != 0 && loop_counts % 50 == 0) {
                    re_request_times = 0;
                    let mut collect_log_string = String::new();
                    let mut res_data_str = String::new();
                    if let Some(str) = data_res.get(0) {
                        res_data_str.push_str(str);
                        res_data_str.push_str("......");
                    } else {
                        res_data_str.push_str("空，请检查接口返回的数据与配置中的映射关系")
                    }

                    match CollectConfigService::cache_data(&state.cache_conn, &data_res).await {
                        Ok(_) => {
                            let log = format!("已累计发起{loop_counts}次请求，本轮采集{}条数据开始插入!\n 处理后的数据为", data_res.len());
                            collect_log_string.push_str(log.as_str());
                            collect_log_string.push_str(res_data_str.as_str());
                        }
                        Err(err) => {
                            let log = format!("已累计发起{loop_counts}次请求，本轮采集{}条数据开始插入!\n 处理后的数据为", data_res.len());
                            collect_log_string.push_str(log.as_str());
                            collect_log_string.push_str(res_data_str.as_str());
                            collect_log_string.push_str("\n");
                            collect_log_string.push_str(err.as_str());
                        }
                    };
                    if let Some(err) = CollectLogService::update_by_id(
                        &state.conn,
                        log_id,
                        collect_log::Model {
                            status: 1,
                            running_log: collect_log_string,
                            ..Default::default()
                        },
                    )
                    .await
                    .err()
                    {
                        error!("status: 1 运行完毕；日志更新失败: {err}");
                    };
                    data_res.clear();
                }
            }
            debug!("分页请求结束, {:?}", data_res);

            return Ok(data_res);
        }
    }

    match collect_data_with_http(data, body_string.clone()).await {
        Ok((_, res)) => {
            let mut collect_log_string = String::new();
            let mut res_data_str = String::new();
            match res.as_ref() {
                Ok(list) => {
                    if let Some(str) = list.get(0) {
                        res_data_str.push_str(str);
                        res_data_str.push_str("......");
                    } else {
                        res_data_str.push_str("空，请检查接口返回的数据与配置中的映射关系")
                    }
                    match CollectConfigService::cache_data(&state.cache_conn, list).await {
                        Ok(_) => {
                            let log =
                                format!("本轮采集{}条数据开始插入!\n 处理后的数据为", list.len());
                            collect_log_string.push_str(log.as_str());
                            collect_log_string.push_str(res_data_str.as_str());
                        }
                        Err(err) => {
                            let log =
                                format!("本轮采集{}条数据开始插入!\n 处理后的数据为", list.len());
                            collect_log_string.push_str(log.as_str());
                            collect_log_string.push_str(res_data_str.as_str());
                            collect_log_string.push_str("\n");
                            collect_log_string.push_str(err.as_str());
                        }
                    };
                }
                Err(err) => {
                    res_data_str = err.to_string();
                    collect_log_string.push_str(res_data_str.as_str());
                }
            }

            if let Some(err) = CollectLogService::update_by_id(
                &state.conn,
                log_id,
                collect_log::Model {
                    status: 2,
                    running_log: collect_log_string,
                    ..Default::default()
                },
            )
            .await
            .err()
            {
                error!("status: 3 运行完毕；日志更新失败: {err}");
            };
            res
        }
        Err(err) => {
            let log = anyhow!("{}", err);
            debug!("{}", log);
            if let Some(err) = CollectLogService::update_by_id(
                &state.conn,
                log_id,
                collect_log::Model {
                    status: 3,
                    running_log: log.to_string(),
                    ..Default::default()
                },
            )
            .await
            .err()
            {
                error!("status: 3 运行完毕；日志更新失败: {err}");
            };

            Err(log)
        }
    }
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
        if let Ok(found_data) = find_value(filed_of_result_data.borrow(), &http_receive.data, false)
        {
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

    if let Some(x) = &data.nested_config {
        let config: Vec<NestedConfig> = serde_json::from_value(x.clone()).unwrap();
        http_receive.set_nested_config(config);
    }

    if let Some(x) = &data.map_rules {
        if !x.as_array().unwrap().is_empty() {
            http_receive.set_map_rules(get_map_rules(Some(x)));
        }
    }

    let res = http_receive
        .serde()?
        .set_template_string(data.template_string.clone())
        .export()
        .await;

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
            .add(Job::new_async_tz(
                format_cron(cron.clone()).as_str(),
                Local::now().timezone(),
                move |_uuid, _l| {
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
