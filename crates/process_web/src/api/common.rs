use axum::Json;
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};

pub type ResJson<T> = Json<ResTemplate<T>>;

pub type ResJsonWithPagination<T> = ResJson<Pagination<Vec<T>>>;

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ResTemplate<T> {
    pub message: String,
    pub data: Option<T>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Pagination<T> {
    pub total: u64,
    pub list: T,
    pub current: u64,
    pub page_size: u64,
}

#[derive(Deserialize, JsonSchema)]
pub struct Id {
    pub id: i32,
}

#[derive(Clone)]
pub struct AppState {
    pub(crate) conn: DatabaseConnection,
}
