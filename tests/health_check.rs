use std::net::TcpListener;
use zero2prod::run;

#[tokio::test]
async fn health_check_works() {
    // Arrange - given
    let address = spawn_app();
    println!("Port gotten >>> {address}");
    let client = reqwest::Client::new();

    // Act - when
    let response = client
        .get(format!("{address}/api/v1/health_check"))
        .send()
        .await
        .expect("Failed to send request");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to bind address");
    tokio::spawn(server);
    format!("http://127.0.0.1:{port}")
}