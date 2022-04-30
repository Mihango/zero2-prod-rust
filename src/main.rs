mod lib;

use std::net::TcpListener;
use crate::lib::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind to port");
    run(listener)?.await
}
