use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct GardenField {
    pub plant: String,
    pub note: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Garden {
    pub id: String,
    pub name: String,
    pub size: u8,
    pub fields: Vec<Option<GardenField>>,
}
