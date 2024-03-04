use anyhow::anyhow;
use std::{env, fmt};
use tracing::error;

#[derive(PartialEq)]
enum AppEnv {
    Dev,
    Prod,
}

impl fmt::Display for AppEnv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppEnv::Dev => write!(f, "dev"),
            AppEnv::Prod => write!(f, "prod"),
        }
    }
}

fn main() {
    let app_env = match env::var("APP_ENV") {
        Ok(v) if v == "prod" => AppEnv::Prod,
        _ => AppEnv::Dev,
    };

    println!("Running in {app_env} mode",);

    match app_env {
        AppEnv::Dev => {
            match dotenvy::dotenv() {
                Ok(path) => println!(".env read successfully from {}", path.display()),
                Err(e) => println!("Could not load .env file: {e}"),
            };
        }
        AppEnv::Prod => {
            match dotenvy::from_filename(".env.prod") {
                Ok(path) => println!(".env.prod read successfully from {}", path.display()),
                Err(e) => println!("Could not load .env.prod file: {e}"),
            };
        }
    }

    if let Some(err) = process_web::api::start().err() {
        error!("Error: {}", anyhow!(err));
    }
}
