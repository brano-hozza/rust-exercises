use axum::{http::StatusCode, Json};
use diesel::RunQueryDsl;

use crate::{establish_connection, schema::users, CreateUser, User};

pub async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    let conn = &mut establish_connection();

    let new_user = diesel::insert_into(users::table)
        .values(&payload)
        .get_result(conn)
        .unwrap();

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(new_user))
}
