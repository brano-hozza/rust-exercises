mod error;
mod garden;
mod plant;
use std::sync::Arc;

use axum::extract::FromRef;
use axum::Router;

type GardenService = Arc<dyn service::garden::GardenService + Send + Sync>;
type PlantService = Arc<dyn service::plant::PlantService + Send + Sync>;

#[derive(FromRef, Clone)]
pub struct RouterState {
    pub garden_service: GardenService,
    pub plant_service: PlantService,
}

pub fn router(state: RouterState) -> Router {
    Router::new()
        .route("/garden", axum::routing::get(garden::get))
        .route("/plants", axum::routing::get(plant::get_all))
        .with_state(state)
}
