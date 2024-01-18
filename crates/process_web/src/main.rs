use anyhow::anyhow;
use tracing::error;

mod api;
mod entity;
mod service;

fn main() {
    if let Some(err) = api::start().err() {
        error!("Error: {}", anyhow!(err));
    }
}
