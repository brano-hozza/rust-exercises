use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{models, AppState};

pub async fn get_garden(State(state): State<AppState>) -> impl IntoResponse {
    // insert your application logic here
    let garden_register = state.garden_register.lock().unwrap();
    (StatusCode::OK, Json(garden_register.clone()))
}

pub async fn get_plants(State(state): State<AppState>) -> impl IntoResponse {
    // insert your application logic here
    let plant_register = state.plant_register.lock().unwrap();
    let plants = plant_register
        .values()
        .map(Clone::clone)
        .collect::<Vec<models::Plant>>();
    (StatusCode::OK, Json(plants))
}

pub async fn add_plant(
    State(state): State<AppState>,
    plant: Json<models::CreatePlantRequestDTO>,
) -> impl IntoResponse {
    // insert your application logic here
    let mut plant_register = state.plant_register.lock().unwrap();
    let id = uuid::Uuid::new_v4().to_string();
    plant_register.insert(
        id.clone(),
        models::Plant {
            id,
            name: plant.name.clone(),
            description: plant.description.clone(),
        },
    );
    (StatusCode::CREATED, Json(()))
}

pub async fn update_field(
    State(state): State<AppState>,
    Path(id): Path<u32>,
    dto: Json<models::UpdateFieldRequestDTO>,
) -> impl IntoResponse {
    // insert your application logic here
    let mut garden_register = state.garden_register.lock().unwrap();
    let field = models::GardenField {
        plant: dto.plant.clone(),
        note: dto.note.clone(),
    };
    garden_register.fields[id as usize] = Some(field);
    (StatusCode::CREATED, Json(()))
}
