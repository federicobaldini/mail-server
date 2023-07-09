use std::net::TcpListener;

mod startup;
mod routes;

use startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
  let listener: TcpListener =
    TcpListener::bind("127.0.0.1:8000").expect("Failed to bind port 8000");
  run(listener)?.await
}
