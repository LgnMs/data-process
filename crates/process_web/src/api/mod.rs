#[macro_use]
pub mod macros;
mod collect_config;
mod collect_log;
pub mod common;
mod mock;

use anyhow::Result;
use axum::Router;
use migration::{Migrator, MigratorTrait};
use sea_orm::*;
use std::env;
use std::sync::Arc;
use axum::http::{StatusCode, Uri};
use tracing::Level;

use crate::api::common::AppState;

#[tokio::main]
pub async fn start() -> Result<()> {
    tracing_subscriber::fmt()
        // all spans/events with a level higher than TRACE (e.g, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::DEBUG)
        .with_line_number(true)
        .with_file(true)
        // sets this to be the default, global subscriber for this application.
        .init();

    env::set_var("RUST_LOG", "debug");
    dotenvy::dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let cache_db_url = env::var("CACHE_DATABASE_URL").expect("CACHE_DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port: String = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");

    let cache_conn = Database::connect(cache_db_url)
        .await
        .expect("Cache Database connection failed");

    // 执行数据库未迁移过任务 在crates/process_web/migration中查看
    Migrator::up(&conn, None).await?;

    let state = Arc::new(AppState { conn, cache_conn });
    // build our application with a route
    let app = Router::new()
        .nest("/collect_config", collect_config::set_routes())
        .nest("/collect_log", collect_log::set_routes())
        .nest("/mock", mock::set_routes())
        .fallback(fallback)
        .with_state(state);

    println!("listener on {server_url}");
    let listener = tokio::net::TcpListener::bind(&server_url).await.unwrap();
    axum::serve(listener, app).await?;

    Ok(())
}

async fn fallback(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("No route for {uri}"))
}
