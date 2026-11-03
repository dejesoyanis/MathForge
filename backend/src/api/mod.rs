mod dto;
mod handlers;

use crate::db::AppState;
use crate::error::AppError;
use axum::extract::DefaultBodyLimit;
use axum::http::{header, HeaderValue, Method, StatusCode};
use axum::routing::{get, post};
use axum::Router;
use tower_http::cors::{AllowOrigin, CorsLayer};

pub fn router(state: AppState, origins: &[String]) -> Result<Router, String> {
    let cors = cors_layer(origins)?;
    Ok(Router::new()
        .route("/api/v1/health", get(handlers::health))
        .route("/api/v1/lessons", get(handlers::list_lessons))
        .route("/api/v1/lessons/{id}", get(handlers::get_lesson))
        .route("/api/v1/lessons/{id}/complete", post(handlers::complete_lesson))
        .route(
            "/api/v1/progress",
            get(handlers::get_progress).delete(handlers::reset_progress),
        )
        .route("/api/v1/exercises/{id}/submit", post(handlers::submit_exercise))
        .route("/api/v1/lab/evaluate", post(handlers::evaluate_lab))
        .route("/api/v1/exam/start", post(handlers::start_exam))
        .route("/api/v1/exam/attempts", get(handlers::list_exam_attempts))
        .route("/api/v1/exam/{id}", get(handlers::get_exam))
        .route("/api/v1/exam/{id}/submit", post(handlers::submit_exam))
        .fallback(handlers::fallback)
        .layer(DefaultBodyLimit::max(32 * 1024))
        .layer(cors)
        .with_state(state))
}

fn cors_layer(origins: &[String]) -> Result<CorsLayer, String> {
    let mut values = Vec::with_capacity(origins.len());
    for origin in origins {
        let value = origin
            .parse::<HeaderValue>()
            .map_err(|_| format!("Ungültiger CORS-Origin «{origin}»."))?;
        values.push(value);
    }
    Ok(CorsLayer::new()
        .allow_origin(AllowOrigin::list(values))
        .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::OPTIONS])
        .allow_headers([header::CONTENT_TYPE]))
}

pub fn bad_json(rejection: axum::extract::rejection::JsonRejection) -> AppError {
    AppError::new(
        StatusCode::BAD_REQUEST,
        "invalid_json",
        "Der JSON-Körper ist ungültig oder passt nicht zum erwarteten Schema.",
        Some(rejection.body_text()),
    )
}
