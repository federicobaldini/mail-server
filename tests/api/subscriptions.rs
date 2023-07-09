use reqwest::{Client, Response};

use crate::helpers::spawn_app;

/// Test function to validate that subscribing with valid form data returns a 200 status code.
#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
  let address: String = spawn_app();
  let client: Client = reqwest::Client::new();
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
