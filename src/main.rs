use std::net::TcpListener;

mod routes;
mod startup;

use mail_server::configuration::{get_configuration, Settings};
use mail_server::startup::run;
use sqlx::{PgPool, Pool, Postgres};

#[tokio::main]
async fn main() -> std::io::Result<()> {
  // Load the application configuration from the environment or configuration file.
  let configuration: Settings = get_configuration().expect("Failed to read configuration.");

  // Connect to the Postgres database using the provided connection string.
  let connection_pool: Pool<Postgres> =
    PgPool::connect(&configuration.database.connection_string())
      .await
      .expect("Failed to connect to Postgres.");

  // Create the address for the server to bind to.
  let address: String = format!("127.0.0.1:{}", configuration.application_port);

  // Bind the TCP listener to the specified address.
  let listener: TcpListener = TcpListener::bind(address)?;

  // Run the server using the TCP listener and connection pool, and await its completion.
  run(listener, connection_pool)?.await
}
