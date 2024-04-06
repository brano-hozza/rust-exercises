use axum::routing::{get, post};
use axum::Router;

use crate::services::user::{create_user, get_users};

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

pub fn init_router() -> Router {
    Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/user", post(create_user))
        .route("/user", get(get_users))
}
