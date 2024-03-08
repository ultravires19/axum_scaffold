use axum::{http::StatusCode, Json};
use serde::Serialize;
use tracing::{event, Level};

#[derive(Serialize)]
pub struct HealthCheckResponse {
    status: String,
}

pub async fn health_check() -> (StatusCode, Json<HealthCheckResponse>) {
    event!(Level::INFO, "health check ran");
    let response = HealthCheckResponse {
        status: "ok".to_string(),
    };
    (StatusCode::OK, Json(response))
}
