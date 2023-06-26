use actix_web::dev::Server;
use reqwest::{Client, Response};

#[tokio::test]
async fn health_check_works() {
  // Start the application server in the background.
  spawn_app();

  // Create a new reqwest client.
  let client: Client = reqwest::Client::new();

  // Send a GET request to the '/health_check' endpoint.
  let response: Response = client
    .get("http://127.0.0.1:8000/health_check")
    .send()
    .await
    .expect("Failed to execute request.");

  // Assert that the response status is successful (2xx).
  assert!(response.status().is_success());

  // Assert that the response content length is 0 (no body).
  assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
  // Start the mail server application and bind it to the address.
  let server: Server = mail_server::run().expect("Failed to bind address");

  // Spawn the server on the tokio runtime in the background.
  let _ = tokio::spawn(server);
}
