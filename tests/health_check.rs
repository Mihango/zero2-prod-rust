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

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let body = "name=John&email=sample%40email.com";
    let response = client
        .post(format!("{address}/api/v1/subscriptions"))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to send request");

    // Assert
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(format!("{address}/api/v1/subscriptions"))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to send request");

        // Assert
        assert_eq!(400, response.status().as_u16(),
                   "The API did not fail with 400 Bad Request when the payload was {}", error_message);
    }
}


// launch the application in the background
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind to port");

    let port = listener.local_addr().unwrap().port();
    println!("Listening on port {}", port);

    let server = zero2prod::startup::run(listener).expect("Failed to spawn app");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{port}")
}
