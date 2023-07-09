use actix_web::{HttpResponse, web};

#[derive(serde::Deserialize)]
pub struct FormData {
  email: String,
  name: String,
}

/// Handler function for the HTTP request on the '/subscriptions' route.
pub async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
  HttpResponse::Ok().finish()
}