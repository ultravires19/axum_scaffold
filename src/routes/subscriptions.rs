use axum::{extract::State, http::StatusCode, Form};
use chrono::Utc;
use sqlx::PgPool;
use tracing::instrument;
use uuid::Uuid;

use crate::domain::{NewSubscriber, SubscriberEmail, SubscriberName};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

#[instrument(
    name = "adding new subscriber",
    skip(form, pool),
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
pub async fn subscribe(State(pool): State<PgPool>, Form(form): Form<FormData>) -> StatusCode {
    // todo: `form.0.try_into()`
    let name = match SubscriberName::parse(form.name) {
        Ok(name) => name,
        Err(_) => return StatusCode::BAD_REQUEST,
    };

    let email = match SubscriberEmail::parse(form.email) {
        Ok(email) => email,
        Err(_) => return StatusCode::BAD_REQUEST,
    };

    let new_subscriber = NewSubscriber { email, name };

    match insert_subscriber(&pool, &new_subscriber).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[instrument(name = "saving subscriber to DB", skip(new_subscriber, pool))]
pub async fn insert_subscriber(
    pool: &PgPool,
    new_subscriber: &NewSubscriber,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
    INSERT INTO subscriptions (id, email, name, subscribed_at)
    VALUES ($1, $2, $3, $4)
            "#,
        Uuid::new_v4(),
        new_subscriber.email.as_ref(),
        new_subscriber.name.as_ref(),
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}
