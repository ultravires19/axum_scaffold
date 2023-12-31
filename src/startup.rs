use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use tokio::net::TcpListener;

use crate::routes::{health_check, subscribe};

pub async fn run(listener: TcpListener, db_pool: PgPool) {
    let app = Router::new()
        .route("/", get(|| async { "hello, World!" }))
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .with_state(db_pool);

    // let server = Server::from_tcp(listener)?.serve(app.into_make_service());
    let server = axum::serve(listener, app).await;

    if let Err(e) = server {
        eprintln!("Server error: {}", e);
    }
}
