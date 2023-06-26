use actix_web::dev::Server;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};

/// Handler function for the HTTP request on the '/health_check' route.
///
/// This function handles the incoming HTTP request and returns a 200 OK response
/// with no body, indicating that the server is healthy.
async fn health_check(_: HttpRequest) -> HttpResponse {
  // Return a 200 OK response with no body indicating the server is healthy.
  HttpResponse::Ok().finish()
}

/// Runs the HTTP server and returns a `Server` instance or an error.
///
/// This function creates an HTTP server, configures the routes, and binds it to
/// the local address and port 8000. It returns a `Server` instance if the binding
/// is successful, otherwise it returns an `std::io::Error`.
pub fn run() -> Result<Server, std::io::Error> {
  // Create an HTTP server and configure routes.
  let server: Server = HttpServer::new(|| {
    App::new().route("/health_check", web::get().to(health_check)) // Define a route for the health check endpoint.
  })
  .bind("127.0.0.1:8000")? // Bind the server to the local address and port 8000.
  .run();

  // Run the server and return the result indicating the success or failure of server binding.
  Ok(server)
}
