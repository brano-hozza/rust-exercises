use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use models::Plant;

pub mod models;
pub mod services;

pub type PlantRegister = Arc<Mutex<HashMap<String, Plant>>>;
pub type GardenRegister = Arc<Mutex<models::Garden>>;

#[derive(Clone)]
pub struct AppState {
    pub plant_register: PlantRegister,
    pub garden_register: GardenRegister,
}
