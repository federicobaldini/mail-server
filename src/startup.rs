use std::net::TcpListener;

use actix_web::{
  dev::Server,
  web::{self, Data},
  App, HttpServer,
};
use sqlx::{PgPool, Pool, Postgres};

use crate::routes::{health_check, subscribe};

/// Runs the HTTP server using the provided TCP listener.
///
/// ### Arguments
///
/// * `listener` - A `TcpListener` instance representing the listener socket.
/// * `db_pool` - A `PgPool` instance representing the PostgreSQL database connection pool.
///
/// ### Returns
///
/// A `Result` containing the `Server` instance if the server started successfully,
/// or an `std::io::Error` if an error occurred.
pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
  // Create a web::Data instance to hold the database connection pool.
  let db_pool: Data<Pool<Postgres>> = web::Data::new(db_pool);

  // Create an HttpServer instance with the provided listener and configure the routes.
  let server: Server = HttpServer::new(move || {
    App::new()
      .route("/health_check", web::get().to(health_check))
      .route("/subscriptions", web::post().to(subscribe))
      .app_data(db_pool.clone())
  })
  .listen(listener)? // Attempts to bind the TCP listener to the server.
  .run(); // Starts the server.

  // Run the server and return the result indicating the success or failure of server binding.
  Ok(server)
}
