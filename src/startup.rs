use std::net::TcpListener;

use actix_web::{App, HttpServer, web};
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use sqlx::PgPool;

use crate::routes::{DefaultGreeter, greet, health_check, subscribe};

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let connection_data = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(DefaultGreeter::new()))
            .app_data(connection_data.clone())
            .service(
                web::scope("/api/v1")
                    .route("/", web::get().to(greet))
                    .route("/{name}", web::get().to(greet))
                    .route("/subscriptions", web::post().to(subscribe))
            )
            .route("/health-check", web::get().to(health_check))
    })
        // .bind(address)?
        .listen(listener)?
        .run();
    Ok(server)
}