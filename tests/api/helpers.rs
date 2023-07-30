use std::net::TcpListener;

use actix_web::dev::Server;
use mail_server::{
  configuration::{get_configuration, DatabaseSettings, Settings},
  startup::run,
};
use sqlx::{Connection, Executor, PgConnection, PgPool, Pool, Postgres};
use uuid::Uuid;

pub struct TestApp {
  pub address: String,
  pub db_pool: PgPool,
  pub db_configuration: DatabaseSettings,
}

/// Spawns the mail server application and returns a TestApp instance.
///
/// ### Returns
/// A `TestApp` instance containing the address of the running mail server and the connection pool to the test database.
pub async fn spawn_app() -> TestApp {
  // Bind the TCP listener to a random available port.
  let listener: TcpListener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

  // Retrieve the port assigned to us by the OS.
  let port: u16 = listener.local_addr().unwrap().port();

  let address: String = format!("http://127.0.0.1:{}", port);

  // Load the application configuration from the environment or configuration file.
  let mut configuration: Settings = get_configuration().expect("Failed to read configuration.");
  // Randomized it to spin up a brand-new logical database for each integration test.
  configuration.database.database_name = Uuid::new_v4().to_string();

  // Connect to the Postgres database using the provided connection string.
  let connection_pool: Pool<Postgres> = configure_database(&configuration.database).await;

  // Start the mail server application and bind it to the address.
  let server: Server = run(listener, connection_pool.clone()).expect("Failed to bind address");

  // Spawn the server on the tokio runtime in the background.
  let _ = tokio::spawn(server);

  // Create a TestApp instance with the address and connection pool.
  TestApp {
    address,
    db_pool: connection_pool,
    db_configuration: configuration.database,
  }
}

/// Configures the database for the mail server application. This function creates a new database
/// with a randomized name based on the provided `DatabaseSettings` configuration, applies
/// database migrations to set up the schema, and returns a connection pool for the newly created database.
///
/// ### Arguments
/// - `config`: A reference to the `DatabaseSettings` configuration representing the database connection details.
///
/// ### Returns
/// A `PgPool` representing the connection pool for the newly created database.
pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
  // Connect to the default Postgres database using the provided connection string (without specifying the database name).
  let mut connection: PgConnection = PgConnection::connect(&config.connection_string_without_db())
    .await
    .expect("Failed to connect to Postgres");

  // Create a new database with the randomized name.
  connection
    .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
    .await
    .expect("Failed to create database.");

  // Connect to the newly created database using the complete connection string, including the randomized database name.
  let connection_pool: Pool<Postgres> = PgPool::connect(&config.connection_string())
    .await
    .expect("Failed to connect to Postgres.");

  // Apply database migrations located in the "./migrations" directory to set up the schema.
  sqlx::migrate!("./migrations")
    .run(&connection_pool)
    .await
    .expect("Failed to migrate the database");

  connection_pool
}