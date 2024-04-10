use crate::error::ApiError;
use crate::RouterState;

use axum::debug_handler;
use axum::extract::State;
use axum::Json;
use model::plant::Plant;

#[debug_handler]
pub async fn get_all(State(state): State<RouterState>) -> Result<Json<Vec<Plant>>, ApiError> {
    state
        .plant_service
        .get_all()
        .await
        .map(Json)
        .map_err(Into::into)
}
