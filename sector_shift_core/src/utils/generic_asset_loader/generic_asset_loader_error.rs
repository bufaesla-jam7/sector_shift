use thiserror::Error;

#[derive(Debug, Error)]
pub enum GenericAssetLoaderError {
    #[error("Io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Ron deserializer error: {0}")]
    RonDe(#[from] ron::error::SpannedError),
}
