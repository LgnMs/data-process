use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use tokio_cron_scheduler::JobScheduler;

pub type ResJson<T> = Json<ResTemplate<T>>;

pub type ResJsonWithPagination<T> = ResJson<Pagination<Vec<T>>>;

#[derive(Serialize, Deserialize)]
pub struct ResTemplate<T> {
    pub message: String,
    pub data: Option<T>,
    pub success: bool
}

#[derive(Serialize, Deserialize)]
pub struct Pagination<T> {
    pub total: u64,
    pub list: T,
    pub current: u64,
    pub page_size: u64,
}

#[derive(Clone)]
pub struct AppState {
    pub(crate) conn: DatabaseConnection,
    pub(crate) cache_conn: DatabaseConnection,
    pub(crate) sched: JobScheduler,
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
        "varchar" | "text" => Some("VARCHAR(255)".to_string()),
        "date" => Some("DATE".to_string()),
        "time" => Some("TIME".to_string()),
        "timestamp" => Some("DATETIME".to_string()),
        _ => None,
    }
}