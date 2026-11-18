#[derive(Debug, thiserror::Error)]
pub enum NexoraError {
    #[error("I/O: {0}")]
    Io(#[from] std::io::Error),
    #[error("configuration: {0}")]
    Configuration(String),
    #[error("lexical error: {0}")]
    Lexical(String),
    #[error("limit exceeded: {0}")]
    Limit(String),
    #[error("invalid path: {0}")]
    InvalidPath(String),
    #[error("storage: {0}")]
    Storage(String),
}
