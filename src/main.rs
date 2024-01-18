use anyhow::anyhow;
use tracing::error;


pub mod scheduler;


fn main() {
    if let Some(err) =  process_web::api::start().err() {
        error!("Error: {}", anyhow!(err));
    }
}
