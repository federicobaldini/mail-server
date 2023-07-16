use reqwest::{Client, Response};

use crate::helpers::spawn_app;
use mail_server::configuration::{get_configuration, Settings};
use sqlx::{Connection, PgConnection};

/// Test function to check if the health check endpoint works.
#[tokio::test]
async fn health_check_works() {
  // Spawn the application and retrieve the address
  let app_address: String = spawn_app();

  // Retrieve the configuration settings
  let configuration: Settings = get_configuration().expect("Failed to read configuration");

  // Build the connection string for the database
  let connection_string: String = configuration.database.connection_string();

  // Connect to the Postgres database
  let connection: PgConnection = PgConnection::connect(&connection_string)
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
}
