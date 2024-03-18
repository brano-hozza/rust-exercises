use axum::{extract::State, http::StatusCode, Json};

use crate::{AppState, CreateUser, User};

pub async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    State(app_state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here

    let result = sqlx::query(
        "
        INSERT INTO users (username)
        VALUES (?)
        ",
    )
    .bind(payload.username.clone())
    .execute(&app_state.pool)
    .await;

    return match result {
        Ok(r) => {
            println!("Inserted {} rows", r.rows_affected());
            (
                StatusCode::CREATED,
                Json(User {
                    id: r.last_insert_rowid(),
                    username: payload.username,
                }),
            )
        }
        Err(e) => {
            println!("Error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json::default())
        }
    };
}
