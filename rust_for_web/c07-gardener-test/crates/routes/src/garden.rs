use crate::error::ApiError;
use crate::RouterState;

use axum::debug_handler;
use axum::extract::State;
use axum::Json;
use model::garden::Garden;

#[debug_handler]
pub async fn get(State(state): State<RouterState>) -> Result<Json<Garden>, ApiError> {
    state
        .garden_service
        .get()
        .await
        .map(|res| Json(res))
        .map_err(|op| op.into())
}
