use std::net::TcpListener;

use actix_web::dev::Server;
use mail_server::startup::run;

/// Spawns the application server and returns the server address.
///
/// ### Returns
///
/// A string representing the address of the spawned server, in the format `http://127.0.0.1:port`.
pub fn spawn_app() -> String {
  let listener: TcpListener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

  // Retrieve the port assigned to us by the OS.
  let port: u16 = listener.local_addr().unwrap().port();

  // Start the mail server application and bind it to the address.
  let server: Server = run(listener).expect("Failed to bind address");

  // Spawn the server on the tokio runtime in the background.
  let _ = tokio::spawn(server);

  format!("http://127.0.0.1:{}", port)
}
