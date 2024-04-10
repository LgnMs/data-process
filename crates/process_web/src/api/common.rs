use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use sea_orm::DatabaseConnection;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio_cron_scheduler::JobScheduler;
use tokio_util::sync::CancellationToken;
use uuid::fmt::Simple;

pub type ResJson<T> = Json<ResTemplate<T>>;

pub type ResJsonWithPagination<T> = ResJson<Pagination<Vec<T>>>;

#[derive(Serialize, Deserialize)]
pub struct ResTemplate<T> {
    pub message: String,
    pub data: Option<T>,
    pub success: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Pagination<T> {
    pub total: u64,
    pub list: T,
    pub current: u64,
    pub page_size: u64,
}

#[derive(Debug, Default)]
pub struct LogTask {
    pub token: CancellationToken,
    pub log_id: i32,
}

impl LogTask {
    pub fn new() -> Self {
        Self {
            token: CancellationToken::new(),
            log_id: -1,
        }
    }

    pub fn set_log_id(&mut self, log_id: i32) -> &Self {
        self.log_id = log_id;
        self
    }
}

#[derive(Clone)]
pub struct AppState {
    pub(crate) conn: DatabaseConnection,
    pub(crate) cache_conn: DatabaseConnection,
    pub(crate) sched: JobScheduler,
    pub(crate) log_task: Arc<RwLock<HashMap<Simple, LogTask>>>,
}

impl AppState {
    pub async fn stop_log_task(&self, log_task_id: Simple) -> Option<i32> {
        let task = self.log_task.read().await;
        let mut log_id = None;
        if let Some(h) = task.get(&log_task_id) {
            h.token.cancel();
            log_id = Some(h.log_id);
            drop(task);
            self.log_task.write().await.remove(&log_task_id);
        };

        log_id
    }
}

// Make our own error that wraps `anyhow::Error`.
#[derive(Debug, Serialize)]
pub struct AppError {
    // 路由需要错误信息也能被序列化
    msg: String,
}

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.msg),
        )
            .into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self {
            msg: err.into().to_string(),
        }
    }
}

#[derive(Deserialize)]
pub struct PaginationPayload<T> {
    pub current: u64,
    pub page_size: u64,
    pub data: Option<T>,
}

pub fn pg_to_mysql_type(pg_type: &str) -> Option<String> {
    match pg_type.to_lowercase().as_str() {
        "smallint" => Some("SMALLINT".to_string()),
        "integer" => Some("INT".to_string()),
        "bigint" => Some("BIGINT".to_string()),
        "decimal" | "numeric" => Some("DECIMAL".to_string()),
        "real" => Some("FLOAT".to_string()),
        "double precision" => Some("DOUBLE".to_string()),
        "boolean" => Some("BOOLEAN".to_string()),
        "char" => Some("CHAR".to_string()),
        "varchar" => Some("VARCHAR(255)".to_string()),
        "date" => Some("DATE".to_string()),
        "time" => Some("TIME".to_string()),
        "timestamp" => Some("DATETIME".to_string()),
        "text" => Some("TEXT".to_string()),
        _ => None,
    }
}

#[derive(Debug)]
pub struct RequestInfo(pub Parts);

impl Serialize for RequestInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut request_info = serializer.serialize_struct("request_info", 5)?;
        request_info.serialize_field("headers", &format!("{:?}", self.0.headers))?;
        request_info.serialize_field("method", &format!("{}", self.0.method))?;
        request_info.serialize_field("uri", &format!("{}", self.0.uri))?;
        request_info.serialize_field("version", &format!("{:?}", self.0.version))?;
        request_info.serialize_field("extensions", &format!("{:?}", self.0.extensions))?;
        request_info.end()
    }
}

// 实现FromRequest trait以从请求中提取信息
#[async_trait]
impl<S> FromRequestParts<S> for RequestInfo
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        Ok(RequestInfo(parts.clone()))
    }
}
