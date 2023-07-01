use std::net::TcpListener;

use actix_web::dev::Server;
use reqwest::{Client, Response};

/// Spawns the application server and returns the server address.
///
/// ### Returns
///
/// A string representing the address of the spawned server, in the format `http://127.0.0.1:port`.
fn spawn_app() -> String {
  let listener: TcpListener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

  // Retrieve the port assigned to us by the OS.
  let port: u16 = listener.local_addr().unwrap().port();

  // Start the mail server application and bind it to the address.
  let server: Server = mail_server::run(listener).expect("Failed to bind address");

  // Spawn the server on the tokio runtime in the background.
  let _ = tokio::spawn(server);

  format!("http://127.0.0.1:{}", port)
}

/// Test function to check if the health check endpoint works.
#[tokio::test]
async fn health_check_works() {
  let address: String = spawn_app();
  let client: Client = reqwest::Client::new();

  // Send a GET request to the '/health_check' endpoint.
  let response: Response = client
    .get(&format!("{}/health_check", &address))
    .send()
    .await
    .expect("Failed to execute request.");

  // Assert that the response status is successful (2xx).
  assert!(response.status().is_success());

  // Assert that the response content length is 0 (no body).
  assert_eq!(Some(0), response.content_length());
}

/// Test function to validate that subscribing with valid form data returns a 200 status code.
#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
  let address: String = spawn_app();
  let client:Client = reqwest::Client::new();
  let body: &str = "name=le%20guin&email=ursula_le_guin%40gmail.com";

  // Send a POST request to the '/subscriptions' endpoint with valid form data.
  let response: Response = client
    .post(&format!("{}/subscriptions", &address))
    .header("Content-Type", "application/x-www-form-urlencoded")
    .body(body)
    .send()
    .await
    .expect("Failed to execute request.");

  // Assert that the response status is 200 OK.
  assert_eq!(200, response.status().as_u16());
}

/// Test function to validate that subscribing with missing data returns a 400 status code.
#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
  let address: String = spawn_app();
  let client: Client = reqwest::Client::new();
  let test_cases: Vec<(&str, &str)> = vec![
    ("name=le%20guin", "missing the email"),
    ("email=ursula_le_guin%40gmail.com", "missing the name"),
    ("", "missing both name and email"),
  ];

  // Iterate over the test cases and send POST requests with missing data.
  for (invalid_body, error_message) in test_cases {
    let response: Response = client
      .post(&format!("{}/subscriptions", &address))
      .header("Content-Type", "application/x-www-form-urlencoded")
      .body(invalid_body)
      .send()
      .await
      .expect("Failed to execute request.");

    // Assert that the response status is 400 Bad Request.
    assert_eq!(
      400,
      response.status().as_u16(),
      "The API did not fail with 400 Bad Request when the payload was {}.",
      error_message
    );
  }
}
