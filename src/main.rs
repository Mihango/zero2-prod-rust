use std::net::TcpListener;

use sqlx::PgPool;

use zero2prod::configurations::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber(String::from("zero2prod"), String::from("info"));
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPool::connect(&configuration.database.connection_string()).await.expect("Failed to connect to database");
    let listener = TcpListener::bind(format!("127.0.0.1:{}", configuration.application_port))
        .expect("Failed to bind to port");
    run(listener, connection_pool)?.await
}
