/// The `Settings` struct represents the overall configuration settings. It has two fields:
/// - `database`: An instance of the `DatabaseSettings` struct, representing the database configuration.
/// - `application_port`: An unsigned 16-bit integer specifying the application's port number.
#[derive(serde::Deserialize)]
pub struct Settings {
  pub database: DatabaseSettings,
  pub application_port: u16,
}

/// The `DatabaseSettings` struct represents the configuration settings for the database. It has the following fields:
/// - `username`: A string representing the username for the database connection.
/// - `password`: A string representing the password for the database connection.
/// - `port`: An unsigned 16-bit integer specifying the port number for the database.
/// - `host`: A string representing the host address of the database server.
/// - `database_name`: A string representing the name of the database.
#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
  pub username: String,
  pub password: String,
  pub port: u16,
  pub host: String,
  pub database_name: String,
}

impl DatabaseSettings {
  /// Returns a connection string for the database based on the configuration settings.
  pub fn connection_string(&self) -> String {
    format!(
      "postgres://{}:{}@{}:{}/{}",
      self.username, self.password, self.host, self.port, self.database_name
    )
  }
  /// Returns a connection string for the database without including the database name.
  pub fn connection_string_without_db(&self) -> String {
    format!(
      "postgres://{}:{}@{}:{}",
      self.username, self.password, self.host, self.port
    )
  }
}

/// The `get_configuration()` function retrieves the configuration settings from a file named "configuration"
/// and returns them as a `Result` containing an instance of the `Settings` struct or a `config::ConfigError` if an error occurs.
/// It uses the `config` crate to manage the configuration.
pub fn get_configuration() -> Result<Settings, config::ConfigError> {
  // Create a new configuration object
  let mut settings = config::Config::default();
  // Merge the configuration file named "configuration" into the settings
  settings.merge(config::File::with_name("configuration"))?;
  // Attempt to convert the merged settings into the Settings struct
  settings.try_into()
}
