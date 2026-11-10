#[derive(Debug, thiserror::Error)]
pub enum ModelError {
    #[error("invalid ciphertext state: {0}")]
    InvalidCiphertext(&'static str),
    #[error("invalid model configuration: {0}")]
    InvalidConfig(&'static str),
}
