use std::net::TcpListener;

fn spawn_app() -> String{
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to port.");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to start server.");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check_works(){
    let address = spawn_app();
    let client = reqwest::Client::new();

    let response = client.get(&format!("{}/health_check", &address)).send().await.expect("Failed to send request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data() {
    //Arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    //Act
    let body = "name=le%20&email=ursula_le_guine%40gmail.com";
    let response = client.post(&format!("{}/subscriptions", &app_address))
    .header("Content-Type", "application/x-www-form-urlencoded")
    .body(body)
    .send()
    .await
    .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guine%40gmail.com", "missing the name"),
        ("","missing both name and email")
    ];
    for (body, expected_message) in test_cases {
        // Act
        let response = client.post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");
        // Assert
        assert_eq!(400, response.status().as_u16(), "Expected a 400 response for test case: {}", expected_message);
    }
}