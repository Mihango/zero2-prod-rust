use actix_web::{web, HttpResponse, Responder, HttpRequest};

pub async fn greet(req: HttpRequest, default_greeter: web::Data<DefaultGreeter>) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or(default_greeter.value.as_str());
    format!("Hello {name}!")
}

pub struct DefaultGreeter {
    value: String,
}

impl DefaultGreeter {
    pub fn new() -> Self {
        DefaultGreeter {
            value: "World".to_string(),
        }
    }
}

// health check
pub(crate) async fn health_check(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}
