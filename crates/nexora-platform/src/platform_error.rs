#[derive(Debug, thiserror::Error)]
pub enum PlatformError {
    #[error("invalid input: {0}")]
    Validation(&'static str),
    #[error("authentication required or credential no longer valid")]
    Unauthorized,
    #[error("operation outside the actor or credential permissions")]
    Forbidden,
    #[error("resource not found")]
    NotFound,
    #[error("conflict: {0}")]
    Conflict(&'static str),
    #[error("platform storage unavailable")]
    Storage,
    #[error("platform capacity unavailable")]
    Unavailable,
    #[error("too many authentication attempts; try again later")]
    TooManyRequests,
}
