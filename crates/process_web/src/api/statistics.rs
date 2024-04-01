use std::collections::HashMap;
use std::sync::Arc;

use anyhow::{anyhow, Result};
use axum::extract::State;
use axum::routing::get;
use axum::{routing::post, Json, Router};
use chrono::{Local, TimeZone};
use sea_orm::{
    ColumnTrait, ConnectionTrait, DbBackend, EntityTrait, FromQueryResult, QueryFilter, QueryOrder,
    QuerySelect, Statement, JsonValue
};
use serde::{Deserialize, Serialize};
use serde::ser::SerializeStruct;
use serde_json::Value;
use sysinfo::{
    DiskUsage, System
};
use migration::Condition;
use ts_rs::TS;

use crate::data_response;
use crate::entity::{
    collect_config, collect_log, data_sharing_config, sharing_request_log, sync_config, sync_log,
};

use super::common::{AppError, AppState, ResJson};

pub fn set_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/collect_task_info", get(collect_task_info))
        .route(
            "/collect_task_info_day_list",
            post(collect_task_info_day_list),
        )
        .route("/sharing_task_info", post(sharing_task_info))
        .route("/sync_task_info", post(sync_task_info))
        .route("/get_sys_info", get(get_sys_info))
        .route("/get_task_info", get(get_task_info))
}

/// 采集数据量总览
#[derive(Serialize, Deserialize, TS)]
#[ts(
    export,
    export_to = "ui/api/models/auto-generates/CollectTaskInfo.ts",
    rename = "CollectTaskInfo"
)]
pub struct CollectTaskInfo {
    pub num_items: i64,
}

pub async fn collect_task_info(
    state: State<Arc<AppState>>,
) -> Result<ResJson<CollectTaskInfo>, AppError> {
    let cache_db = &state.cache_conn;
    let query_table_name_sql: &str = match cache_db.get_database_backend() {
        DbBackend::MySql => {
             "show tables;"
        }
        DbBackend::Postgres => {
            "SELECT table_name FROM information_schema.tables WHERE table_schema = 'public';"
        }
        _ => {
            return Err(anyhow!("不支持的数据库格式").into());
        }
    };

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

#[derive(Serialize, Deserialize, TS)]
#[ts(
    export,
    export_to = "ui/api/models/auto-generates/CollectTaskInfoDayListReq.ts",
    rename = "CollectTaskInfoDayListReq"
)]
pub struct CollectTaskInfoDayListReq {
    pub date: [i64; 2],
}

#[derive(Serialize, Deserialize, TS)]
#[ts(
    export,
    export_to = "ui/api/models/auto-generates/CollectTaskInfoRes.ts",
    rename = "CollectTaskInfoRes"
)]
pub struct CollectTaskInfoRes {
    #[ts(type = "any")]
    list: Vec<Value>,
    #[ts(type = "any")]
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
            collect_log::Column::UpdateTime.gte(
                Local
                    .timestamp_millis_opt(payload.date[0])
                    .unwrap()
                    .naive_local(),
            ),
        )
        .add(
            collect_log::Column::UpdateTime.lte(
                Local
                    .timestamp_millis_opt(payload.date[1])
                    .unwrap()
                    .naive_local(),
            ),
        );


    let query: &str = match state.conn.get_database_backend() {
        DbBackend::MySql => {
            "SELECT DATE_FORMAT(update_time, '%Y-%m-%d') AS date, COUNT(id) AS num_items FROM collect_log GROUP BY date ORDER BY date;"
        }
        DbBackend::Postgres => {
            "SELECT TO_CHAR(update_time, 'YYYY-MM-DD') AS date, COUNT(id) AS num_items FROM collect_log GROUP BY date ORDER BY date;"
        }
        _ => {
            return Err(anyhow!("不支持的数据库格式").into());
        }
    };

    let list = JsonValue::find_by_statement(
        Statement::from_sql_and_values(state.conn.get_database_backend(), query, []),
    )
        .all(&state.conn)
        .await?;

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
        .limit(10)
        .into_json()
        .all(&state.conn)
        .await?;

    let res: Result<CollectTaskInfoRes> = Ok(CollectTaskInfoRes {
        list,
        rank_list,
    });

    data_response!(res)
}

#[derive(Serialize, Deserialize, TS)]
#[ts(
    export,
    export_to = "ui/api/models/auto-generates/SharingTaskInfoReq.ts",
    rename = "SharingTaskInfoReq"
)]
pub struct SharingTaskInfoReq {
    pub date: [i64; 2],
}

#[derive(Serialize, Deserialize, TS)]
#[ts(
    export,
    export_to = "ui/api/models/auto-generates/SharingTaskInfoRes.ts",
    rename = "SharingTaskInfoRes"
)]
pub struct SharingTaskInfoRes {
    list: HashMap<String, i32>,
    num_items: i64,
    #[ts(type = "any")]
    rank_list: Vec<Value>,
    user_number: i32,
    avg_num_user_calls_api: i32,
}

#[derive(FromQueryResult, Default, TS)]
#[ts(
    export,
    export_to = "ui/api/models/auto-generates/NumItems.ts",
    rename = "NumItems"
)]
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
            sharing_request_log::Column::UpdateTime.gte(
                Local
                    .timestamp_millis_opt(payload.date[0])
                    .unwrap()
                    .naive_local(),
            ),
        )
        .add(
            sharing_request_log::Column::UpdateTime.lte(
                Local
                    .timestamp_millis_opt(payload.date[1])
                    .unwrap()
                    .naive_local(),
            ),
        );

    let list = sharing_request_log::Entity::find()
        .filter(conditions.clone())
        .order_by_desc(sharing_request_log::Column::UpdateTime)
        .all(&state.conn)
        .await?;

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

    let avg_num_user_calls_api = if !user_calls_times.is_empty() {
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
        .limit(10)
        .into_json()
        .all(&state.conn)
        .await?;

    let res: Result<SharingTaskInfoRes> = Ok(SharingTaskInfoRes {
        list: info_day_map,
        num_items,
        rank_list,
        avg_num_user_calls_api,
        user_number: user_calls_times.len() as i32,
    });

    data_response!(res)
}

#[derive(Serialize, Deserialize, TS)]
#[ts(
    export,
    export_to = "ui/api/models/auto-generates/SyncTaskInfoReq.ts",
    rename = "SyncTaskInfoReq"
)]
pub struct SyncTaskInfoReq {
    pub date: [i64; 2],
}

#[derive(Serialize, Deserialize, TS)]
#[ts(
    export,
    export_to = "ui/api/models/auto-generates/SyncTaskInfoRes.ts",
    rename = "SyncTaskInfoRes"
)]
pub struct SyncTaskInfoRes {
    list: HashMap<String, i32>,
    num_items: i64,
    #[ts(type = "any")]
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
            sync_log::Column::UpdateTime.gte(
                Local
                    .timestamp_millis_opt(payload.date[0])
                    .unwrap()
                    .naive_local(),
            ),
        )
        .add(
            sync_log::Column::UpdateTime.lte(
                Local
                    .timestamp_millis_opt(payload.date[1])
                    .unwrap()
                    .naive_local(),
            ),
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
        .limit(10)
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

struct MyDiskUsage(DiskUsage);

impl Serialize for MyDiskUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer 
    {
        let mut disk_usage = serializer.serialize_struct("disk_usage", 4)?;
        disk_usage.serialize_field("total_written_bytes", &format!("{:?}", self.0.total_written_bytes))?;
        disk_usage.serialize_field("written_bytes", &format!("{}", self.0.written_bytes))?;
        disk_usage.serialize_field("total_read_bytes", &format!("{}", self.0.total_read_bytes))?;
        disk_usage.serialize_field("read_bytes", &format!("{:?}", self.0.read_bytes))?;
        disk_usage.end()
    }
}

#[derive(Serialize, TS)]
#[ts(
    export,
    export_to = "ui/api/models/auto-generates/SystemInfo.ts",
    rename = "SystemInfo"
)]
struct SystemInfo {
    total_memory: u64,
    used_memory: u64,
    total_swap: u64,
    used_swap: u64,
    cpu_uses: Vec<f32>,
    processes_cpu_usage: Option<f32>,
    #[ts(type = "any")]
    processes_disk_usage: Option<MyDiskUsage>,
    processes_memory_usage: Option<u64>,
}

async fn get_sys_info() -> Result<ResJson<SystemInfo>, AppError> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let (processes_disk_usage, processes_cpu_usage, processes_memory_usage) = {
        let mut value = None;
        for (_, process) in sys.processes() {
            if process.name().contains("data_process") {
                value = Some((process.disk_usage(), process.cpu_usage(), process.memory()));
            }
        }
        if let Some(x) = value {
            (Some(x.0), Some(x.1), Some(x.2))
        } else {
            (None, None, None)
        }
    };

    let my_processes_disk_usage = match processes_disk_usage {
        Some(x) => Some(MyDiskUsage(x)),
        None => None,
    };
    
    let res: Result<SystemInfo> = Ok(SystemInfo {
        total_memory: sys.total_memory(),
        used_memory: sys.used_memory(),
        total_swap: sys.total_swap(),
        used_swap: sys.used_swap(),
        cpu_uses: sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect(),
        processes_disk_usage: my_processes_disk_usage,
        processes_cpu_usage,
        processes_memory_usage
    });

    data_response!(res)
}

#[derive(Serialize, TS)]
#[ts(
    export,
    export_to = "ui/api/models/auto-generates/TaskInfo.ts",
    rename = "TaskInfo"
)]
struct TaskInfo {
    collect_num: i64,
    sync_num: i64,
    sharing_num: i64
}

async fn get_task_info(state: State<Arc<AppState>>) -> Result<ResJson<TaskInfo>, AppError> {
    let collect_num = collect_config::Entity::find()
        .select_only()
        .column_as(collect_config::Column::Id.count(), "num_items")
        .into_model::<NumItems>()
        .one(&state.conn)
        .await?
        .unwrap_or_default();
    let sync_num = sync_config::Entity::find()
        .select_only()
        .column_as(sync_config::Column::Id.count(), "num_items")
        .into_model::<NumItems>()
        .one(&state.conn)
        .await?
        .unwrap_or_default();
    let sharing_num = data_sharing_config::Entity::find()
        .select_only()
        .column_as(data_sharing_config::Column::Id.count(), "num_items")
        .into_model::<NumItems>()
        .one(&state.conn)
        .await?
        .unwrap_or_default();

    let res: Result<TaskInfo> = Ok(TaskInfo {
        collect_num: collect_num.num_items,
        sync_num: sync_num.num_items,
        sharing_num: sharing_num.num_items
    });

    data_response!(res)
}