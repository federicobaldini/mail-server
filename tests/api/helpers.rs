use std::net::TcpListener;

use actix_web::dev::Server;
use mail_server::{
  configuration::{get_configuration, Settings},
  startup::run,
};
use sqlx::{PgPool, Pool, Postgres};

pub struct TestApp {
  pub address: String,
  pub db_pool: PgPool,
}

/// Spawns the mail server application and returns a TestApp instance.
pub async fn spawn_app() -> TestApp {
  // Bind the TCP listener to a random available port.
  let listener: TcpListener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

  // Retrieve the port assigned to us by the OS.
  let port: u16 = listener.local_addr().unwrap().port();

  let address: String = format!("http://127.0.0.1:{}", port);

  // Load the application configuration from the environment or configuration file.
  let configuration: Settings = get_configuration().expect("Failed to read configuration.");

  // Connect to the Postgres database using the provided connection string.
  let connection_pool: Pool<Postgres> =
    PgPool::connect(&configuration.database.connection_string())
      .await
      .expect("Failed to connect to Postgres.");

  // Start the mail server application and bind it to the address.
  let server: Server = run(listener, connection_pool.clone()).expect("Failed to bind address");

  // Spawn the server on the tokio runtime in the background.
  let _ = tokio::spawn(server);

  // Create a TestApp instance with the address and connection pool.
  TestApp {
    address,
    db_pool: connection_pool,
  }
}
