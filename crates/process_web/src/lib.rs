/// fn main() {
///     if let Some(err) = api::start().err() {
///         error!("Error: {}", anyhow!(err));
///     }
/// }
pub mod api;
mod data_source;
pub mod entity;
pub mod service;
pub mod utils;
