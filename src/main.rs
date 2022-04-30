use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(DefaultGreeter::new()))
            .service(
            web::scope("/api/v1")
                .route("/", web::get().to(greet))
                .route("/{name}", web::get().to(greet))
        )
            .route("/health-check", web::get().to(health_check))
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}

async fn greet(req: HttpRequest, default_greeter: web::Data<DefaultGreeter>) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or(default_greeter.value.as_str());
    format!("Hello {name}!")
}

struct DefaultGreeter {
    value: String,
}

impl DefaultGreeter {
    fn new() -> Self {
        DefaultGreeter {
            value: "World".to_string(),
        }
    }
}

// health check
async fn health_check(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}
