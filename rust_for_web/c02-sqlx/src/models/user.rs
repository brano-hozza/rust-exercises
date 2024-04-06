use serde::Serialize;

// the output to our `create_user` handler
#[derive(Serialize)]
pub struct User {
    pub id: i64,
    pub username: String,
}

impl Default for User {
    fn default() -> Self {
        User {
            id: 0,
            username: "".to_string(),
        }
    }
}
