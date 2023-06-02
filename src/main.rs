use actix_web::{web, App, HttpRequest, HttpServer, Responder};

// Handler function for the HTTP request.
async fn greet(request: HttpRequest) -> impl Responder {
  let name: &str = request.match_info().get("name").unwrap_or("World");
  format!("Hello {}!", &name)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
  // Create an HTTP server and configure routes.
  HttpServer::new(|| {
    App::new()
      .route("/", web::get().to(greet)) // Route for the root path.
      .route("/{name}", web::get().to(greet)) // Route with a dynamic parameter.
  })
  .bind("127.0.0.1:8000")? // Bind the server to a specific address and port.
  .run() // Run the server.
  .await // Wait for the server to finish.
}
