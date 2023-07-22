use reqwest::{Client, Response};

use crate::helpers::{spawn_app, TestApp};

/// Test function to validate that subscribing with valid form data returns a 200 status code.
#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
  // Spawn the application and retrieve an instance of TestApp
  let app: TestApp = spawn_app().await;

  // Create a new reqwest client
  let client: Client = reqwest::Client::new();

  // Prepare the request body
  let body: &str = "name=federico%20baldini&email=federico_baldini%40gmail.com";

  // Send a POST request to the subscriptions endpoint
  let response: Response = client
    .post(&format!("{}/subscriptions", &app.address))
    .header("Content-Type", "application/x-www-form-urlencoded")
    .body(body)
    .send()
    .await
    .expect("Failed to execute request.");

  // Assert that the response status is 200
  assert_eq!(200, response.status().as_u16());

  // Fetch the saved subscription from the database
  let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
    .fetch_one(&app.db_pool)
    .await
    .expect("Failed to fetch saved subscription.");

  // Assert that the saved subscription matches the provided form data
  assert_eq!(saved.email, "federico_baldini@gmail.com");
  assert_eq!(saved.name, "federico baldini");
}

/// Test function to validate that subscribing with missing data returns a 400 status code.
#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
  // Spawn the application and retrieve an instance of TestApp
  let app: TestApp = spawn_app().await;

  // Create a new reqwest client
  let client: Client = reqwest::Client::new();

  // Define the test cases, where each test case consists of an invalid request body and an error message.
  let test_cases: Vec<(&str, &str)> = vec![
    ("name=federico%20baldini", "missing the email"),
    ("email=federico_baldini%40gmail.com", "missing the name"),
    ("", "missing both name and email"),
  ];

  // Iterate over the test cases and send POST requests with missing data.
  for (invalid_body, error_message) in test_cases {
    let response: Response = client
      .post(&format!("{}/subscriptions", &app.address))
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
