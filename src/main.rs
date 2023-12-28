use sqlx::PgPool;
// use std::error::Error;
use tokio::net::TcpListener;
use tracing::{event, Level};
use zero2axum::configuration::get_configuration;
use zero2axum::startup::run;
use zero2axum::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() {
    let subscriber = get_subscriber("info".into());
    init_subscriber(subscriber);
    event!(Level::INFO, "yeahhh babyy");
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).await.unwrap();
    run(listener, connection_pool).await;
}
