use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GardenField {
    pub plant: String,
    pub note: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Garden {
    pub name: String,
    pub size: u16,
    pub fields: Vec<Option<GardenField>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Plant {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreatePlantRequestDTO {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateFieldRequestDTO {
    pub plant: String,
    pub note: Option<String>,
}
