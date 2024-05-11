// use once_cell::sync::Lazy;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::sync::Once;
use tokio::net::TcpListener;
use uuid::Uuid;
use zero2axum::configuration::{get_configuration, DatabaseSettings};
use zero2axum::email_client::EmailClient;
use zero2axum::startup;
use zero2axum::telemetry::{get_subscriber, init_subscriber};

static INIT: Once = Once::new();

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

pub async fn spawn_app() -> TestApp {
    INIT.call_once(|| {
        let subscriber = get_subscriber("info".into());
        init_subscriber(subscriber);
    });
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("failed to bind port");
    let port = listener.local_addr().unwrap().port();
    // let server = zero2axum::startup::run(listener).expect("failed to bind address");
    let address = format!("http://127.0.0.1:{}", port);

    let mut configuration = get_configuration().expect("Failed to read configuration.");
    configuration.database.database_name = Uuid::new_v4().to_string();
    let connection_pool = configure_database(&configuration.database).await;

    let sender_email = configuration
        .email_client
        .sender()
        .expect("invalid sender email address");

    let timeout = configuration.email_client.timeout();
    let email_client = EmailClient::new(
        configuration.email_client.base_url,
        sender_email,
        configuration.email_client.authorization_token,
        timeout,
    );

    let server = startup::run(listener, connection_pool.clone(), email_client);
    let _ = tokio::spawn(server);
    TestApp {
        address,
        db_pool: connection_pool,
    }
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create database
    let mut connection = PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("Failed to connect to Postgres while configuring database");
    connection
        .execute(&*format!(r#"CREATE DATABASE "{}";"#, config.database_name))
        .await
        .expect("Failed to create database.");

    // Migrate database
    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}
