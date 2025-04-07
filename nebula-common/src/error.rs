#[derive(Debug, thiserror::Error)]
pub enum NebulaError {
    #[error(transparent)]
    System(#[from] SystemError),

    #[error(transparent)]
    Config(#[from] ConfigError),
}

#[derive(Debug, thiserror::Error)]
pub enum SystemError {
    #[error(transparent)]
    StdIo(#[from] std::io::Error),
    
    #[error(transparent)]
    Arti(#[from] arti_client::Error),
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
    Arti
}

pub type Result<T> = std::result::Result<T, NebulaError>;
