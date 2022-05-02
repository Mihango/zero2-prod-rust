use actix_web::{web, Responder};

// #[post("/api/v1/subscriptions")]
pub(crate) async fn subscribe_user(form: web::Form<FormData>) -> impl Responder {
    format!("Welcome {}!", form.name)
}

#[derive(Debug, serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}
