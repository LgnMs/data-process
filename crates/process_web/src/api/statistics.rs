use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

use anyhow::{anyhow, Result};
use axum::extract::State;
use axum::{routing::post, Json, Router};
use chrono::NaiveDateTime;
use sea_orm::prelude::DateTime;
use sea_orm::{ColumnTrait, ConnectionTrait, DbBackend, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, Statement};
use serde::{Deserialize, Serialize};

use migration::Condition;

use crate::data_response;
use crate::entity::{collect_log, sharing_request_log};

use super::common::{AppError, AppState, ResJson};

/// 1. 总采集量
///     - 通过采集任务采集到的数据总量
///     - 时间维度下每日采集任务执行次数（面积图形式展示）
///     - 当日执行的采集任务次数
/// 2. 总访问数
///     - 共享接口被调用的次数
///     - 时间维度下每日被调用的次数（面积图形式展示）
///     - 当日被调用的次数
/// 3. 总同步次数
///     - 同步任务运行次数
///     - 时间维度下每日运行次数（面积图形式展示）
///     - 当日运行次数
/// 4. 性能监测
///     - CPU占比
///     - 内存占用
/// 5. 任务执行状况概览
///     - 年、月、日采集量统计
///     - 年、月、日同步任务运行次数统计
///     - 前x名运行次数的采集任务
///     - 前x名运行次数的同步任务
/// 6. 共享接口调用
///     - 访问用户数
///     - 人均调用次数
///     - 前x名接口调用
///         - 调用量
/// 7. 任务占比
///     - 三种类型任务再系统中的占比详情

pub fn set_routes() -> Router<Arc<AppState>> {
    let routes = Router::new()
        .route("/collect_task_info", post(collect_task_info))
        .route(
            "/collect_task_info_day_list",
            post(collect_task_info_day_list),
        )
        .route(
            "/sharing_task_info",
            post(sharing_task_info),
        );

    routes
}

/// 采集数据量总览
#[derive(Serialize, Deserialize)]
pub struct CollectTaskInfo {
    pub num_items: i64,
}

struct CollectRunInfo {
    id: i32,
    name: String,
    number: i32,
}

pub async fn collect_task_info(
    state: State<Arc<AppState>>,
) -> Result<ResJson<CollectTaskInfo>, AppError> {
    let cache_db = &state.cache_conn;
    let query_table_name_sql: &str;

    match cache_db.get_database_backend() {
        DbBackend::MySql => {
            query_table_name_sql = "show tables;";
        }
        DbBackend::Postgres => {
            query_table_name_sql =
                "SELECT table_name FROM information_schema.tables WHERE table_schema = 'public';";
        }
        _ => {
            return Err(anyhow!("不支持的数据库格式").into());
        }
    }

    let table_list = cache_db
        .query_all(Statement::from_string(
            cache_db.get_database_backend(),
            query_table_name_sql,
        ))
        .await?;

    let mut num_items: i64 = 0;
    for item in table_list {
        let table_name: String = item.try_get_by_index(0)?;

        let count = cache_db
            .query_one(Statement::from_string(
                cache_db.get_database_backend(),
                format!("select count(id) from {}", table_name),
            ))
            .await?;

        if count.is_none() {
            return Err(anyhow!("未找到表 {}", table_name).into());
        }
        let number: i64 = count.unwrap().try_get_by_index(0)?;
        num_items += number;
    }

    let res: Result<CollectTaskInfo> = Ok(CollectTaskInfo { num_items });

    data_response!(res)
}

#[derive(Serialize, Deserialize)]
pub struct CollectTaskInfoDayListReq {
    pub date: [i64; 2],
}

/// 获取每日采集任务执行的次数
pub async fn collect_task_info_day_list(
    state: State<Arc<AppState>>,
    Json(payload): Json<CollectTaskInfoDayListReq>,
) -> Result<ResJson<HashMap<String, i32>>, AppError> {
    let mut conditions = Condition::all();
    conditions = conditions
        .add(collect_log::Column::UpdateTime.gte(NaiveDateTime::from_timestamp_millis(payload.date[0])))
        .add(collect_log::Column::UpdateTime.lte(NaiveDateTime::from_timestamp_millis(payload.date[1])));

    let list = collect_log::Entity::find()
        .filter(conditions)
        .order_by_desc(collect_log::Column::UpdateTime)
        .all(&state.conn)
        .await?;

    let mut info_day_map: HashMap<String, i32> = HashMap::new();

    for item in list {
        let date = item.update_time.format("%Y-%m-%d").to_string();

        info_day_map
            .entry(date)
            .and_modify(|number| *number += 1)
            .or_insert(1);
    }

    let res: Result<HashMap<String, i32>> = Ok(info_day_map);
    data_response!(res)
}


#[derive(Serialize, Deserialize)]
pub struct SharingTaskInfoReq {
    pub date: [i64; 2],
}

#[derive(Serialize, Deserialize)]
pub struct SharingTaskInfoRes {
    list: HashMap<String, i32>,
    num_items: i64
}

/// 获取共享任务调用详情
pub async fn sharing_task_info(
    state: State<Arc<AppState>>,
    Json(payload): Json<SharingTaskInfoReq>,
) -> Result<ResJson<SharingTaskInfoRes>, AppError> {
    let mut conditions = Condition::all();
    conditions = conditions
        .add(sharing_request_log::Column::UpdateTime.gte(NaiveDateTime::from_timestamp_millis(payload.date[0])))
        .add(sharing_request_log::Column::UpdateTime.lte(NaiveDateTime::from_timestamp_millis(payload.date[1])));

    let list = sharing_request_log::Entity::find()
        .filter(conditions)
        .order_by_desc(sharing_request_log::Column::UpdateTime)
        .all(&state.conn)
        .await?;

    let mut info_day_map: HashMap<String, i32> = HashMap::new();

    for item in list {
        let date = item.update_time.format("%Y-%m-%d").to_string();

        info_day_map
            .entry(date)
            .and_modify(|number| *number += 1)
            .or_insert(1);
    }


    let count_res = state.conn
        .query_one(Statement::from_string(
            state.conn.get_database_backend(),
            "select COUNT(*) as num_items from sharing_request_log ",
        ))
        .await?;

    let num_items: i64 = {
        if let Some(count_res) = count_res {
            count_res.try_get_by_index(0)?
        } else {
            0
        }
    };

    // sharing_request_log::Entity::find()
    //     .group_by(sharing_request_log::Column::DataSharingConfigId)
    //     .all(&state.conn)
    //     .await?;

    let res: Result<SharingTaskInfoRes> = Ok(SharingTaskInfoRes {
        list: info_day_map,
        num_items
    });

    data_response!(res)
}