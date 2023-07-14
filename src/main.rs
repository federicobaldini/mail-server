use std::net::TcpListener;

mod routes;
mod startup;

use mail_server::configuration::{get_configuration, Settings};
use mail_server::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
  let configuration: Settings = get_configuration().expect("Failed to read configuration.");
  let address: String = format!("127.0.0.1:{}", configuration.application_port);
  let listener: TcpListener = TcpListener::bind(address)?;
  run(listener)?.await
}
