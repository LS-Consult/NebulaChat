#[derive(Debug, thiserror::Error)]
pub enum NebulaError {
    #[error(transparent)]
    System(#[from] SystemError),

    #[error(transparent)]
    Config(#[from] ConfigError),

    #[error(transparent)]
    Bonk(#[from] BonkError),
}

#[derive(Debug, thiserror::Error)]
pub enum SystemError {
    #[error(transparent)]
    StdIo(#[from] std::io::Error),

    #[error(transparent)]
    Arti(#[from] arti_client::Error),

    #[error("Failed to start reverse proxy")]
    ArtiReverseProxy,
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Config file not found")]
    NotFound,

    #[error(transparent)]
    Unpacking(#[from] toml::de::Error),

    #[error(transparent)]
    Packing(#[from] toml::ser::Error),

    #[error("Arti config error")]
    Arti,
}

#[derive(Debug, thiserror::Error)]
pub enum BonkError {
    #[error("No handshake was completed")]
    HandshakeIncomplete,

    #[error("Malformed frame")]
    MalformedFrame,

    #[error("Failed to write a data stream")]
    WriterFailure,

    #[error(transparent)]
    Serialization(#[from] ciborium::ser::Error<std::io::Error>),
}

pub type Result<T> = std::result::Result<T, NebulaError>;
