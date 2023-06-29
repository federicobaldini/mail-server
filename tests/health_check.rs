use std::net::TcpListener;

use actix_web::dev::Server;
use reqwest::{Client, Response};

/// Asynchronous test function to check if the health check endpoint works.
#[tokio::test]
async fn health_check_works() {
  // Start the application server in the background.
  let address: String = spawn_app();

  // Create a new reqwest client.
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
