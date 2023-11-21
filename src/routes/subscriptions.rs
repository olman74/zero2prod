use actix_web::{HttpResponse, web};
use sqlx::PgPool;
use sqlx::types::uuid;
use uuid::Uuid;
use tracing::Instrument;
use chrono::Utc;


#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(_form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!(
        "Adding a new subscriber.",
        %request_id,
        subscriber_email =  %_form.email,
        subscriber_name = %_form.name
    );

    let _request_span_guard = request_span.enter();

    let query_span = tracing::info_span!("Saving new subscriber details in the database");

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
        .execute(pool.as_ref())
        .instrument(query_span)
        .await {
        Ok(_) => {
            HttpResponse::Ok().finish()
        },
        Err(e) => {
            tracing::error!("Failed to execute query {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}