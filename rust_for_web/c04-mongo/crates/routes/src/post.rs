use crate::error::ApiError;
use crate::RouterState;

use axum::debug_handler;
use axum::extract::Path;
use axum::extract::State;
use axum::Json;
use axum_extra::extract::WithRejection;
use model::post::Post;

#[debug_handler]
pub async fn get_all(State(state): State<RouterState>) -> Result<Json<Vec<Post>>, ApiError> {
    state
        .post_service
        .get_all()
        .await
        .map(Json)
        .map_err(Into::into)
}

pub async fn get(
    State(state): State<RouterState>,
    WithRejection(Path(id), _): WithRejection<Path<String>, ApiError>,
) -> Result<Json<Post>, ApiError> {
    state
        .post_service
        .get(id)
        .await
        .map(Json)
        .map_err(Into::into)
}

pub async fn post(
    State(state): State<RouterState>,
    WithRejection(Json(post), _): WithRejection<Json<Post>, ApiError>,
) -> Result<Json<Post>, ApiError> {
    state
        .post_service
        .new(post)
        .await
        .map(Json)
        .map_err(Into::into)
}

pub async fn delete(
    State(state): State<RouterState>,
    WithRejection(Path(id), _): WithRejection<Path<String>, ApiError>,
) -> Result<(), ApiError> {
    state.post_service.delete(id).await.map_err(Into::into)
}
