use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Display;
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiErrorPayload {
    pub code: &'static str,
    pub message: String,
}

#[derive(Error, Debug)]
pub struct ApiError {
    pub status: StatusCode,
    pub payload: ApiErrorPayload,
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.status, self.payload.message)
    }
}

impl From<service::error::ServiceError> for ApiError {
    fn from(value: service::error::ServiceError) -> Self {
        match value {
            all @ service::error::ServiceError::DatabaseError(_) => ApiError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                payload: ApiErrorPayload {
                    code: "database_failure",
                    message: format!("{all}"),
                },
            },
            all @ service::error::ServiceError::NotFound => ApiError {
                status: StatusCode::NOT_FOUND,
                payload: ApiErrorPayload {
                    code: "not_found",
                    message: format!("{all}"),
                },
            },
        }
    }
}

// // Either do this or use `thiserror`.
// macro_rules! impl_from_rejection_for_apierror {
//     ($rejection:ty,$code:expr, $status:expr) => {
//         impl From<$rejection> for ApiError {
//             fn from(value: $rejection) -> Self {
//                 ApiError {
//                     status: $status,
//                     payload: ApiErrorPayload {
//                         code: $code,
//                         message: format!("{value}"),
//                     },
//                 }
//             }
//         }
//     };
// }

// impl_from_rejection_for_apierror!(
//     axum::extract::rejection::PathRejection,
//     "bad_request",
//     StatusCode::BAD_REQUEST
// );
// impl_from_rejection_for_apierror!(
//     axum::extract::rejection::JsonRejection,
//     "bad_request",
//     StatusCode::BAD_REQUEST
// );

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        (self.status, Json(self.payload)).into_response()
    }
}
