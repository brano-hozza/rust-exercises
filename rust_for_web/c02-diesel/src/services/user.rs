use axum::{http::StatusCode, Json};
use diesel::RunQueryDsl;

use crate::{establish_connection, schema::users, CreateUser, User};

pub async fn get_users() -> (StatusCode, Json<Vec<User>>) {
    let conn = &mut establish_connection();

    let users = users::table
        .load::<User>(conn)
        .expect("Error loading users");

    // this will be converted into a JSON response
    // with a status code of `200 OK`
    (StatusCode::OK, Json(users))
}

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
