use std::net::TcpListener;

use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use actix_web::dev::Server;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .service(web::scope("/api/v1")
                // web::get is a shortcut of Route::new().guard(guard::GET())
                .route("/health_check", web::get().to(health_check))
            )
    })
        .listen(listener)?
        .run();
    Ok(server)
}

async fn health_check() -> impl Responder {
    println!("Calling health check endpoint");
    HttpResponse::Ok().finish()
}