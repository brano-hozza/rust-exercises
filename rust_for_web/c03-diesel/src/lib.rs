use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use std::env;

use models::user::*;
use router::init_router;

mod models;
mod router;
mod schema;
mod services;

pub fn establish_connection() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub async fn start() {
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = init_router();

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
