use std::net::TcpListener;

use actix_web::{dev::Server, HttpServer, web, App};

use crate::routes::{health_check, subscribe};

/// Runs the HTTP server using the provided TCP listener.
///
/// ### Arguments
///
/// * `listener` - A `TcpListener` instance representing the listener socket.
///
/// ### Returns
///
/// A `Result` containing the `Server` instance if the server started successfully,
/// or an `std::io::Error` if an error occurred.
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
  let server: Server = HttpServer::new(|| {
    App::new()
      .route("/health_check", web::get().to(health_check))
      .route("/subscriptions", web::post().to(subscribe))
  })
  .listen(listener)? // Attempts to bind the TCP listener to the server.
  .run(); // Starts the server.

  // Run the server and return the result indicating the success or failure of server binding.
  Ok(server)
}
