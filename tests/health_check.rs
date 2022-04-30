use std::fmt::format;
use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();
    // need to bring in reqwest to make http request
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("{address}/health-check"))
        .send()
        .await.expect("Failed to send request");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}


// launch the application in the background
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind to port");

    let port = listener.local_addr().unwrap().port();
    println!("Listening on port {}", port);

    let server = zero2prod::run(listener).expect("Failed to spawn app");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{port}")
}
