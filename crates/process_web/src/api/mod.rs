use anyhow::Result;
use axum::http::{StatusCode, Uri};
use axum::{middleware, Router};
use migration::{Migrator, MigratorTrait};
use sea_orm::*;
use std::env;
use std::sync::Arc;
use tokio_cron_scheduler::JobScheduler;
use tower::ServiceBuilder;
use tracing::Level;
use tracing_appender::rolling;
use tracing_subscriber::fmt::writer::MakeWriterExt;

use crate::api::auth::jwt_middleware;
use crate::api::common::AppState;
use crate::service::collect_config_service::CollectConfigService;
use crate::service::sync_config_service::SyncConfigService;

mod auth;
pub mod collect_config;
pub mod collect_log;
pub mod common;
pub mod data_sharing_config;
pub mod data_source_list;
pub mod mock;
pub mod sharing_request_log;
pub mod statistics;
pub mod sync_config;
pub mod sync_log;

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

    let sched = JobScheduler::new().await?;

    let state = Arc::new(AppState {
        conn,
        cache_conn,
        sched,
    });

    // 初始化调度任务
    CollectConfigService::setup_collect_config_cron(&state).await?;
    SyncConfigService::setup_collect_config_cron(&state).await?;
    state.sched.start().await?;

    // build our application with a route
    let app = Router::new()
        .nest("/auth", auth::set_routes())
        .nest("/collect_config", collect_config::set_routes())
        .nest("/collect_log", collect_log::set_routes())
        .nest("/sync_config", sync_config::set_routes())
        .nest("/sync_log", sync_log::set_routes())
        .nest("/data_source_list", data_source_list::set_routes())
        .nest("/data_sharing_config", data_sharing_config::set_routes())
        .nest("/sharing_request_log", sharing_request_log::set_routes())
        .nest("/statistics", statistics::set_routes())
        .nest("/mock", mock::set_routes())
        .fallback(fallback)
        .layer(ServiceBuilder::new().layer(middleware::from_fn(jwt_middleware)))
        .with_state(state);

    println!("listener on {server_url}");
    let listener = tokio::net::TcpListener::bind(&server_url).await.unwrap();
    axum::serve(listener, app).await?;

    Ok(())
}

fn setup_log() {
    let builder = tracing_subscriber::fmt();

    match env::var("APP_ENV") {
        Ok(v) if v == "prod" => {
            let debug_file = rolling::daily("./logs", "debug");
            let warn_file =
                rolling::daily("./logs", "warnings").with_max_level(tracing::Level::WARN);
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

async fn fallback(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("No route for {uri}"))
}
