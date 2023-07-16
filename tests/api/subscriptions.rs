use reqwest::{Client, Response};

use crate::helpers::spawn_app;
use mail_server::configuration::{get_configuration, Settings};
use sqlx::{Connection, PgConnection};

/// Test function to validate that subscribing with valid form data returns a 200 status code.
#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
  // Spawn the application and retrieve the address
  let app_address: String = spawn_app();

  // Retrieve the configuration settings
  let configuration: Settings = get_configuration().expect("Failed to read configuration");

  // Build the connection string for the database
  let connection_string: String = configuration.database.connection_string();

  // Connect to the Postgres database
  let mut connection: PgConnection = PgConnection::connect(&connection_string)
    .await
    .expect("Failed to connect to Postgres.");

  // Create a new reqwest client
  let client: Client = reqwest::Client::new();

  // Prepare the request body
  let body: &str = "name=federico%20baldini&email=federico_baldini%40gmail.com";

  // Send a POST request to the subscriptions endpoint
  let response: Response = client
    .post(&format!("{}/subscriptions", &app_address))
    .header("Content-Type", "application/x-www-form-urlencoded")
    .body(body)
    .send()
    .await
    .expect("Failed to execute request.");

  // Assert that the response status is 200
  assert_eq!(200, response.status().as_u16());

  let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
    .fetch_one(&mut connection)
    .await
    .expect("Failed to fetch saved subscription.");
  
  assert_eq!(saved.email, "federico_baldini@gmail.com");
  assert_eq!(saved.name, "federico baldini");
}

/// Test function to validate that subscribing with missing data returns a 400 status code.
#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
  let address: String = spawn_app();
  let client: Client = reqwest::Client::new();
  let test_cases: Vec<(&str, &str)> = vec![
    ("name=federico%20baldini", "missing the email"),
    ("email=federico_baldini%40gmail.com", "missing the name"),
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
