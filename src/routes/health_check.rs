use actix_web::{HttpResponse, HttpRequest};

/// Handler function for the HTTP request on the '/health_check' route.
///
/// This function handles the incoming HTTP request and returns a 200 OK response
/// with no body, indicating that the server is healthy.
pub async fn health_check(_: HttpRequest) -> HttpResponse {
  HttpResponse::Ok().finish()
}