use reqwest::{Client, Response};

use crate::helpers::spawn_app;

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
