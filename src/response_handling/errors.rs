use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::response_handling::endpoint_response::ErrorResponse;

pub enum ApplicationErrors {
    InternalError,
    NotFound,
}

impl IntoResponse for ApplicationErrors {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApplicationErrors::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            ),
            ApplicationErrors::NotFound => (StatusCode::NOT_FOUND, "Resource Not Found".to_owned()),
        };

        (status, Json(ErrorResponse::new(message))).into_response()
    }
}
