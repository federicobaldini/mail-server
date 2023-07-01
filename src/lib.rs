use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};

#[derive(serde::Deserialize)]
struct FormData {
  email: String,
  name: String,
}

/// Handler function for the HTTP request on the '/health_check' route.
///
/// This function handles the incoming HTTP request and returns a 200 OK response
/// with no body, indicating that the server is healthy.
async fn health_check(_: HttpRequest) -> HttpResponse {
  HttpResponse::Ok().finish()
}

/// Handler function for the HTTP request on the '/subscriptions' route.
async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
  HttpResponse::Ok().finish()
}

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
