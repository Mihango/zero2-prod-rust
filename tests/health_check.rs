use zero2prod::run;

#[tokio::test]
async fn health_check_works() {
    // Arrange - given
    spawn_app();
    let client = reqwest::Client::new();

    // Act - when
    let response = client
        .get("http://0.0.0.0:8080/api/v1/health_check")
        .send()
        .await
        .expect("Failed to send request");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
    let server = run().expect("Failed to bind address");
    tokio::spawn(server);
}