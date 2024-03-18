use std::env;

use dtos::user::*;
use models::user::*;
use router::init_router;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};

mod dtos;
mod models;
mod router;
mod services;

#[derive(Clone)]
struct AppState {
    pool: Pool<Sqlite>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // initialize tracing
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let app_state = AppState {
        pool: SqlitePoolOptions::new()
            .max_connections(5)
            .connect(
                env::var("DATABASE_URL")
                    .unwrap_or("sqlite::memory:".to_string())
                    .as_str(),
            )
            .await?,
    };

    // build our application with a route
    let app = init_router(app_state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
