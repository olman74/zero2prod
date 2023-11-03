use actix_web::{HttpResponse, web};
use chrono::Utc;
use sqlx::PgPool;
use sqlx::types::uuid;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(_form: web::Form<FormData>, _connection: web::Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();
    log::info!(
        "Request id {} - Adding '{}' '{}' as a new subscriber.",
        request_id,
        _form.email,
        _form.name
    );

    log::info!("Request id {} - Saving new subscriber details in the database", request_id);

    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1,$2, $3, $4)
        "#,
        Uuid::new_v4(),
        _form.email,
        _form.name,
        Utc::now()
    )
        .execute(_connection.get_ref())
        .await {
        Ok(_) => {
            log::info!("Request id {} - New subscriber details have been saved", request_id);
            HttpResponse::Ok().finish()
        },
        Err(e) => {
            log::error!("Request id {} - Failed to execute query: {:?}", request_id, e);
            HttpResponse::InternalServerError().finish()
        }
    }
}