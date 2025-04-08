use crate::error::{ConfigError, Result, SystemError};

/// Represents the configuration for a client.
///
/// This structure encapsulates the client's essential configuration data,
/// including the username and the Ed25519 keypair.
///
/// # Fields
///
/// - `username`
///   The unique username associated with the client.
/// - `keypair`
///   The Ed25519 keypair used for cryptographic operations. The keypair is
///   stored as a `Vec<u8>`, where it contains both the private and public
///   keys.
///
/// This structure is both serializable and deserializable using
/// the `serde` framework, allowing seamless integration with
/// configuration formats like TOML.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ClientConfig {
    pub username: String,
    pub keypair: Vec<u8>,
}

impl ClientConfig {
    /// Loads the client configuration from the default path.
    ///
    /// This function attempts to load a `ClientConfig` instance by reading
    /// a `config.toml` file located in the user's configuration directory,
    /// specifically under a subdirectory named `nebula`.
    ///
    /// # Returns
    ///
    /// - `Ok(ClientConfig)` if the configuration file is successfully found,
    ///   read, and deserialized.
    /// - `Err(ConfigError::NotFound)` if the configuration file does not exist.
    /// - `Err(ConfigError::Unpacking)` if there is an error during deserialization
    ///   of the TOML content.
    /// - `Err(SystemError::StdIo)` if there is an I/O error while accessing the file
    ///   or determining the config directory path.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use nebula_common::config::client::ClientConfig;
    ///
    /// match ClientConfig::load_from_default_path() {
    ///     Ok(config) => println!("Configuration loaded successfully: {:?}", config),
    ///     Err(e) => eprintln!("Failed to load configuration: {}", e),
    /// }
    /// ```
    pub fn load_from_default_path() -> Result<Self> {
        let mut config_dir = dirs::config_dir()
            .ok_or(std::io::Error::from(std::io::ErrorKind::NotFound))
            .map_err(SystemError::StdIo)?;

        config_dir.push("nebula");
        config_dir.push("config.toml");

        if !config_dir.exists() {
            return Err(ConfigError::NotFound.into());
        }

        let config_file_result = std::fs::read_to_string(config_dir);
        match config_file_result {
            Ok(config_file_content) => {
                let config_result = toml::from_str::<ClientConfig>(&config_file_content);
                match config_result {
                    Ok(config) => Ok(config),
                    Err(e) => Err(ConfigError::Unpacking(e).into()),
                }
            }
            Err(e) => Err(SystemError::StdIo(e).into()),
        }
    }

    /// Saves the client configuration to the default path.
    ///
    /// This function attempts to save the current instance of `ClientConfig`
    /// to a `config.toml` file located in the user's configuration directory,
    /// specifically under a subdirectory named `nebula`.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the configuration is successfully serialized and written to the file.
    /// - `Err(ConfigError::Packing)` if there is an error during the serialization of the
    ///   `ClientConfig` instance to the TOML format.
    /// - `Err(SystemError::StdIo)` if there is an I/O error while accessing the file
    ///   or determining the config directory path.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use nebula_common::config::client::ClientConfig;
    ///
    /// let config = ClientConfig {
    ///     username: "example_user".to_string(),
    ///     keypair: vec![1, 2, 3, 4],
    /// };
    ///
    /// match config.save_to_default_path() {
    ///     Ok(_) => println!("Configuration saved successfully."),
    ///     Err(e) => eprintln!("Failed to save configuration: {}", e),
    /// }
    /// ```
    pub fn save_to_default_path(&self) -> Result<()> {
        let mut config_dir = dirs::config_dir()
            .ok_or(std::io::Error::from(std::io::ErrorKind::NotFound))
            .map_err(SystemError::StdIo)?;

        config_dir.push("nebula");
        config_dir.push("config.toml");

        let config_content_result = toml::to_string_pretty(self);
        match config_content_result {
            Ok(config_content) => match std::fs::write(config_dir, config_content) {
                Ok(_) => Ok(()),
                Err(e) => Err(SystemError::StdIo(e).into()),
            },
            Err(e) => Err(ConfigError::Packing(e).into()),
        }
    }
}
