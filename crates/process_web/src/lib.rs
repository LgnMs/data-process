/// fn main() {
///     if let Some(err) = api::start().err() {
///         error!("Error: {}", anyhow!(err));
///     }
/// }
pub mod api;
pub mod entity;
pub mod service;
pub mod utils;
