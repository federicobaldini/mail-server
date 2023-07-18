use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
  email: String,
  name: String,
}

/// Handles the HTTP POST request for subscribing to a service.
///
/// ### Arguments
///
/// * `form` - A `web::Form` containing the form data submitted in the request body.
/// * `pool` - A `web::Data` instance holding the PostgreSQL connection pool.
///
/// ### Returns
///
/// An `HttpResponse` indicating the success or failure of the subscription operation.
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
  // Execute an SQL query to insert the form data into the 'subscriptions' table.
  match sqlx::query!(
    r#"
  INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1, $2, $3, $4)
  "#,
    Uuid::new_v4(),
    form.email,
    form.name,
    Utc::now()
  )
  .execute(pool.as_ref())
  .await
  {
    // Return a successful HTTP response.
    Ok(_) => HttpResponse::Ok().finish(),
    // If there was an error executing the query, log the error and return an internal server error response.
    Err(e) => {
      println!("Failed to execute query: {}", e);
      HttpResponse::InternalServerError().finish()
    }
  }
}
