use std::net::TcpListener;

use actix_web::{App, HttpServer, web};
use actix_web::dev::Server;

use crate::routes::{DefaultGreeter, greet, health_check, subscribe_user};

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(DefaultGreeter::new()))
            .service(
                web::scope("/api/v1")
                    .route("/", web::get().to(greet))
                    .route("/{name}", web::get().to(greet))
                    .route("/subscriptions", web::post().to(subscribe_user))
            )
            .route("/health-check", web::get().to(health_check))
    })
        // .bind(address)?
        .listen(listener)?
        .run();
    Ok(server)
}
