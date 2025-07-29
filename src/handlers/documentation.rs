use std::fs;

use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    Json,
};

use crate::handlers::responses::MessageResponse;

pub async fn handle_serve_documentation() -> Response {
    match fs::read_to_string("public/docs.html") {
        Ok(html) => return (StatusCode::OK, Html(html)).into_response(),
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(MessageResponse::new("Failed to find docs.html")),
            )
                .into_response();
        }
    }
}
