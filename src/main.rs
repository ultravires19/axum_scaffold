use sqlx::PgPool;
// use std::error::Error;
use tokio::net::TcpListener;
use tracing::{event, Level};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};
use zero2axum::configuration::get_configuration;
use zero2axum::email_client::EmailClient;
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
    let sender_email = configuration
        .email_client
        .sender()
        .expect("Invalid sender email address.");
    let timeout = configuration.email_client.timeout();
    let email_client = EmailClient::new(
        configuration.email_client.base_url,
        sender_email,
        configuration.email_client.authorization_token,
        timeout,
    );

    let address = format!("127.0.0.1:{}", configuration.application.port);
    let listener = TcpListener::bind(address).await.unwrap();
    run(listener, connection_pool, email_client).await;
}
