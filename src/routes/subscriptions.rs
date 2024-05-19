use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::{query, PgPool};
use uuid::Uuid;
use tracing;

use crate::domain::{NewSubsciber, SubscriberName};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}
#[tracing::instrument(
    name = "Adding a new subscriber", 
    skip(form,pool), 
    fields(subscriber_email = %form.email, subscriber_name=%form.name))]

pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    let new_subscriber = NewSubsciber {email : form.0.email, 
        name : SubscriberName::parse(form.0.name).expect("Name validation failed")};

    match insert_subscriber(&pool, &new_subscriber)
    .await
    {
        Ok(_) => {
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[tracing::instrument(name="Saving new subscriber details in database",skip(new_subscriber,pool))]

pub async fn insert_subscriber(pool: &PgPool, new_subscriber: &NewSubsciber)-> Result<(), sqlx::Error> {
    sqlx::query!(r#"
    INSERT INTO subscriptions (id, email, name, subscribed_at)
    VALUES ($1, $2, $3, $4)"#,
        Uuid::new_v4(),
        new_subscriber.email,
        new_subscriber.name.as_ref(),
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {tracing::error!("Failed to execute query: {:?}", e);e})?;
    Ok(())
}
