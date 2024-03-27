use std::collections::HashMap;
use std::sync::Arc;

use anyhow::{anyhow, Result};
use axum::extract::State;
use axum::{routing::post, Json, Router};
use chrono::{Local, TimeZone};
use sea_orm::{
    ColumnTrait, ConnectionTrait, DbBackend, EntityTrait, FromQueryResult, QueryFilter, QueryOrder,
    QuerySelect, Statement,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use migration::Condition;

use crate::data_response;
use crate::entity::{
    collect_config, collect_log, data_sharing_config, sharing_request_log, sync_config, sync_log,
};

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
        .route("/sharing_task_info", post(sharing_task_info))
        .route("/sync_task_info", post(sync_task_info));

    routes
}

/// 采集数据量总览
#[derive(Serialize, Deserialize)]
pub struct CollectTaskInfo {
    pub num_items: i64,
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

#[derive(Serialize, Deserialize)]
pub struct CollectTaskInfoRes {
    list: HashMap<String, i32>,
    rank_list: Vec<Value>,
}

/// 获取每日采集任务执行的次数
pub async fn collect_task_info_day_list(
    state: State<Arc<AppState>>,
    Json(payload): Json<CollectTaskInfoDayListReq>,
) -> Result<ResJson<CollectTaskInfoRes>, AppError> {
    let mut conditions = Condition::all();
    conditions = conditions
        .add(
            sharing_request_log::Column::UpdateTime
                .gte(Local.timestamp_millis_opt(payload.date[0]).unwrap().naive_local()),
        )
        .add(
            sharing_request_log::Column::UpdateTime
                .lte(Local.timestamp_millis_opt(payload.date[1]).unwrap().naive_local()),
        );

    let list = collect_log::Entity::find()
        .filter(conditions.clone())
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

    let rank_list: Vec<Value> = collect_log::Entity::find()
        .select_only()
        .column(collect_log::Column::CollectConfigId)
        .column(collect_config::Column::Name)
        .column_as(collect_log::Column::Id.count(), "num_items")
        .inner_join(collect_config::Entity)
        .filter(conditions)
        .group_by(collect_log::Column::CollectConfigId)
        .group_by(collect_config::Column::Name)
        .order_by_desc(collect_log::Column::Id.count())
        .limit(15)
        .into_json()
        .all(&state.conn)
        .await?;

    let res: Result<CollectTaskInfoRes> = Ok(CollectTaskInfoRes {
        list: info_day_map,
        rank_list,
    });

    data_response!(res)
}

#[derive(Serialize, Deserialize)]
pub struct SharingTaskInfoReq {
    pub date: [i64; 2],
}

#[derive(Serialize, Deserialize)]
pub struct SharingTaskInfoRes {
    list: HashMap<String, i32>,
    num_items: i64,
    rank_list: Vec<Value>,
    user_number: i32,
    avg_num_user_calls_api: i32,
}

#[derive(FromQueryResult)]
struct NumItems {
    num_items: i64,
}

/// 获取共享任务调用详情
pub async fn sharing_task_info(
    state: State<Arc<AppState>>,
    Json(payload): Json<SharingTaskInfoReq>,
) -> Result<ResJson<SharingTaskInfoRes>, AppError> {
    let mut conditions = Condition::all();
    conditions = conditions
        .add(
            sharing_request_log::Column::UpdateTime
                .gte(Local.timestamp_millis_opt(payload.date[0]).unwrap().naive_local()),
        )
        .add(
            sharing_request_log::Column::UpdateTime
                .lte(Local.timestamp_millis_opt(payload.date[1]).unwrap().naive_local()),
        );

    let list = sharing_request_log::Entity::find()
        .filter(conditions.clone())
        .order_by_desc(sharing_request_log::Column::UpdateTime)
        .all(&state.conn)
        .await?;

        print!("list {list:?} list");
    let mut info_day_map = HashMap::new();
    let mut user_calls_times = HashMap::new();

    for item in list {
        let date = item.update_time.format("%Y-%m-%d").to_string();

        info_day_map
            .entry(date)
            .and_modify(|number| *number += 1)
            .or_insert(1);

        if let Some(user_info) = item.user_info {
            let info: Value = serde_json::from_str(user_info.as_str())?;
            if let Some(user) = info.get("user") {
                let key = user.as_str().unwrap_or_default();
                user_calls_times
                    .entry(key.to_string())
                    .and_modify(|number| *number += 1)
                    .or_insert(1);
            }

        }
    }

    let mut sum_calls_times = 0;
    for item in &user_calls_times {
        sum_calls_times += item.1;
    }

    let avg_num_user_calls_api = if     user_calls_times.len() != 0 {
        sum_calls_times / user_calls_times.len() as i32
    } else {
        0
    };

    let count_res = sharing_request_log::Entity::find()
        .select_only()
        .column_as(sharing_request_log::Column::Id.count(), "num_items")
        .into_model::<NumItems>()
        .one(&state.conn)
        .await?;

    let num_items: i64 = {
        if let Some(count_res) = count_res {
            count_res.num_items
        } else {
            0
        }
    };

    let rank_list: Vec<Value> = sharing_request_log::Entity::find()
        .select_only()
        .column(sharing_request_log::Column::DataSharingConfigId)
        .column(data_sharing_config::Column::Name)
        .column_as(sharing_request_log::Column::Id.count(), "num_items")
        .inner_join(data_sharing_config::Entity)
        .filter(conditions)
        .group_by(sharing_request_log::Column::DataSharingConfigId)
        .group_by(data_sharing_config::Column::Name)
        .order_by_desc(sharing_request_log::Column::Id.count())
        .limit(15)
        .into_json()
        .all(&state.conn)
        .await?;

    let res: Result<SharingTaskInfoRes> = Ok(SharingTaskInfoRes {
        list: info_day_map,
        num_items,
        rank_list,
        avg_num_user_calls_api,
        user_number: user_calls_times.len() as i32
    });

    data_response!(res)
}

#[derive(Serialize, Deserialize)]
pub struct SyncTaskInfoReq {
    pub date: [i64; 2],
}

#[derive(Serialize, Deserialize)]
pub struct SyncTaskInfoRes {
    list: HashMap<String, i32>,
    num_items: i64,
    rank_list: Vec<Value>,
}

/// 获取同步任务调用详情
pub async fn sync_task_info(
    state: State<Arc<AppState>>,
    Json(payload): Json<SyncTaskInfoReq>,
) -> Result<ResJson<SyncTaskInfoRes>, AppError> {
    let mut conditions = Condition::all();
    conditions = conditions
        .add(
            sharing_request_log::Column::UpdateTime
                .gte(Local.timestamp_millis_opt(payload.date[0]).unwrap().naive_local()),
        )
        .add(
            sharing_request_log::Column::UpdateTime
                .lte(Local.timestamp_millis_opt(payload.date[1]).unwrap().naive_local()),
        );

    let list = sync_log::Entity::find()
        .filter(conditions.clone())
        .order_by_desc(sync_log::Column::UpdateTime)
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

    let count_res = sync_log::Entity::find()
        .select_only()
        .column_as(sync_log::Column::Id.count(), "num_items")
        .into_model::<NumItems>()
        .one(&state.conn)
        .await?;

    let num_items: i64 = {
        if let Some(count_res) = count_res {
            count_res.num_items
        } else {
            0
        }
    };

    let rank_list: Vec<Value> = sync_log::Entity::find()
        .select_only()
        .column(sync_log::Column::SyncConfigId)
        .column(sync_config::Column::Name)
        .column_as(sync_log::Column::Id.count(), "num_items")
        .inner_join(sync_config::Entity)
        .filter(conditions)
        .group_by(sync_log::Column::SyncConfigId)
        .group_by(sync_config::Column::Name)
        .order_by_desc(sync_log::Column::Id.count())
        .limit(15)
        .into_json()
        .all(&state.conn)
        .await?;

    let res: Result<SyncTaskInfoRes> = Ok(SyncTaskInfoRes {
        list: info_day_map,
        num_items,
        rank_list,
    });

    data_response!(res)
}
