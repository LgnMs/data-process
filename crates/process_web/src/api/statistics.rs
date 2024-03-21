use std::sync::Arc;

use anyhow::{anyhow, Result};
/// 1. 总采集量
///     - 通过采集任务采集到的数据总量
///     - 时间维度下每日采集的数据量（面积图形式展示）
///     - 当日采集数据量
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
use axum::{Json, Router, routing::post};
use axum::extract::State;
use sea_orm::{ConnectionTrait, DbBackend, Statement};
use sea_orm::prelude::DateTime;
use serde::{Deserialize, Serialize};

use crate::data_response;

use super::common::{AppError, AppState, ResJson};

pub fn set_routes() -> Router<Arc<AppState>> {
    let routes = Router::new().route("/collect_running_info", post(collect_running_info));

    routes
}


/// 采集数据量总览
#[derive(Serialize, Deserialize)]
struct CollectRunningInfo {
    sum_total: i64,
    // list: Vec<CollectRunningInfoDay>,
    // number_of_runs_list: Vec<CollectRunInfo>
}

/// 每日采集的数据量
struct CollectRunningInfoDay {
    sum_total: i32,
    date: DateTime
}

struct CollectRunInfo {
    id: i32,
    name: String,
    number: i32,
}

#[derive(Serialize, Deserialize)]
struct CollectRunningInfoReq {
}

pub async fn collect_running_info(state: State<Arc<AppState>>, Json(payload): Json<CollectRunningInfoReq>,) -> Result<ResJson<CollectRunningInfo>, AppError> {
    let cache_db = &state.cache_conn;
    let query_table_name_sql: &str;

    match cache_db.get_database_backend() {
        DbBackend::MySql => {
            query_table_name_sql = "show tables;";
        }
        DbBackend::Postgres => {
            query_table_name_sql = "SELECT table_name FROM information_schema.tables WHERE table_schema = 'public';";
        }
        _ => {
            return Err(anyhow!("不支持的数据库格式").into());
        }
    }

    let table_list = cache_db
        .query_all(Statement::from_string(
            cache_db.get_database_backend(),
            query_table_name_sql
        ))
        .await?;

    let mut sum_total: i64 = 0;
    for item in table_list {
        let table_name: String = item.try_get_by_index(0)?;

        let count = cache_db
            .query_one(Statement::from_string(
                cache_db.get_database_backend(),
                format!("select count(id) from {}", table_name)
            ))
            .await?;

        if count.is_none() {
            return Err(anyhow!("未找到表 {}", table_name).into());
        }
        let number: i64 = count.unwrap().try_get_by_index(0)?;
        sum_total += number;
    }

    let res: Result<CollectRunningInfo> = Ok(CollectRunningInfo {
        sum_total
    });

    data_response!(res)
}
