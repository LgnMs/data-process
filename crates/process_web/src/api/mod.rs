#[macro_use]
pub mod macros;
pub mod collect_config;
pub mod collect_log;
pub mod common;
pub mod mock;

use anyhow::Result;
use axum::http::{StatusCode, Uri};
use axum::Router;
use migration::{Migrator, MigratorTrait};
use sea_orm::*;
use std::env;
use std::sync::Arc;
use tracing::Level;
use tracing_appender::rolling;
use tracing_subscriber::fmt::writer::MakeWriterExt;

use crate::api::common::AppState;

fn setup_log() {

    let builder = tracing_subscriber::fmt();

    match env::var("APP_ENV") {
        Ok(v) if v == "prod" => {
            let debug_file = rolling::daily("./logs", "debug");
            let warn_file = rolling::daily("./logs", "warnings").with_max_level(tracing::Level::WARN);
            let all_files = debug_file.and(warn_file);
            builder
                .with_writer(all_files)
                .with_max_level(Level::DEBUG)
                .with_line_number(true)
                .with_file(true)
                .init();

            println!("日志存储于./logs");
        }
        _ => {
            builder
                .with_max_level(Level::DEBUG)
                .with_line_number(true)
                .with_file(true)
                .init();
        }
    };
}

#[tokio::main]
pub async fn start() -> Result<()> {
    env::set_var("RUST_LOG", "debug");
    dotenvy::dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let cache_db_url =
        env::var("CACHE_DATABASE_URL").expect("CACHE_DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port: String = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    setup_log();

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
