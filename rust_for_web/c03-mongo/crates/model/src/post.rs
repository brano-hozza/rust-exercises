use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Post {
    pub id: Option<String>,
    pub name: String,
    pub content: String,
}
