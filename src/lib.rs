use std::net::TcpListener;

use actix_web::{App, HttpRequest, HttpResponse, HttpServer, post, Responder, web};
use actix_web::dev::Server;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(DefaultGreeter::new()))
            .service(
                web::scope("/api/v1")
                    .route("/", web::get().to(greet))
                    .route("/{name}", web::get().to(greet))
                    .route("subscriptions", web::post().to(subscribe_user))
            )
            .route("/health-check", web::get().to(health_check))
    })
        // .bind(address)?
        .listen(listener)?
        .run();
    Ok(server)
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


// #[post("/api/v1/subscriptions")]
async fn subscribe_user(form: web::Form<FormData>) -> impl Responder {
    format!("Welcome {}!", form.name)
}


#[derive(Debug, serde::Deserialize)]
struct FormData {
    name: String,
    email: String,
}
