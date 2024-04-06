use crate::schema::users;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Insertable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: i32,
    pub username: String,
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: 0,
            username: "default".to_string(),
        }
    }
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct CreateUser {
    pub username: String,
}
