#[macro_use]
pub mod macros;
mod collect_config;
mod collect_log;
pub mod common;

use std::env;
use std::sync::Arc;

use aide::{
    axum::{routing::get, ApiRouter, IntoApiResponse},
    openapi::{Info, OpenApi},
};
use anyhow::Result;
use axum::response::Html;
use axum::{Extension, Json};
use migration::{Migrator, MigratorTrait};
use sea_orm::*;
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
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port: String = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");

    Migrator::up(&conn, None).await?;

    let state = Arc::new(AppState { conn });
    // build our application with a route
    let app = ApiRouter::new()
        .route(
            "/swagger",
            get(|| async { Html(axum_swagger_ui::swagger_ui("/api.json")) }),
        )
        .route("/api.json", get(serve_api))
        .nest("/collect_config", collect_config::set_routes())
        .nest("/collect_log", collect_log::set_routes())
        .with_state(state);

    let mut api = OpenApi {
        info: Info {
            description: Some("API".to_string()),
            ..Info::default()
        },
        ..OpenApi::default()
    };

    println!("listener on {server_url}");
    let listener = tokio::net::TcpListener::bind(&server_url).await.unwrap();
    axum::serve(
        listener,
        app
            // Generate the documentation.
            .finish_api(&mut api)
            // Expose the documentation to the handlers.
            .layer(Extension(api))
            .into_make_service(),
    )
    .await?;

    Ok(())
}

// Note that this clones the document on each request.
// To be more efficient, we could wrap it into an Arc,
// or even store it as a serialized string.
async fn serve_api(Extension(api): Extension<OpenApi>) -> impl IntoApiResponse {
    Json(api)
}
