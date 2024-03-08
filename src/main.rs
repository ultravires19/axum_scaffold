use sqlx::PgPool;
// use std::error::Error;
use tokio::net::TcpListener;
use tracing::{event, Level};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};
use zero2axum::configuration::get_configuration;
use zero2axum::startup::run;
// use zero2axum::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry().with(fmt::layer()).init();
    event!(Level::INFO, "let's gooo!");
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).await.unwrap();
    run(listener, connection_pool).await;
}
