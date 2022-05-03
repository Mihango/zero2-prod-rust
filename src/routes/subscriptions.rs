use actix_web::{HttpResponse, Responder, web};
use sqlx::PgPool;

use chrono::Utc;
use uuid::Uuid;

// #[post("/api/v1/subscriptions")]
pub async fn subscribe_user(form: web::Form<FormData>,
                            pool: web::Data<PgPool>) -> impl Responder {
    format!("Welcome {} >>> {}!", form.name, form.email);
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
        // use get_ref to get an immutable reference to the connection
        .execute(pool.get_ref())
        .await {
        Ok(_) =>  HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute query >>> {:?}", e);
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }

}

#[derive(Debug, serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}
