//! Error types for the BONK Staking client library

use thiserror::Error;

/// Result type for BONK staking operations
pub type Result<T> = std::result::Result<T, BonkStakingError>;

/// Errors that can occur when using the BONK Staking client
#[derive(Debug, Error)]
pub enum BonkStakingError {
    /// Error from the Solana client
    #[error("Solana client error: {0}")]
    ClientError(#[from] solana_client::client_error::ClientError),

    /// Failed to deserialize account data
    #[error("Failed to deserialize account data")]
    DeserializationError,

    /// Failed to serialize data
    #[error("Failed to serialize data: {0}")]
    SerializationError(std::io::Error),

    /// Account not found
    #[error("Account not found: {0}")]
    AccountNotFound(String),

    /// Invalid account data
    #[error("Invalid account data: {0}")]
    InvalidAccountData(String),

    /// Transaction failed
    #[error("Transaction failed: {0}")]
    TransactionFailed(String),

    /// Insufficient balance
    #[error("Insufficient balance: required {required}, available {available}")]
    InsufficientBalance { required: u64, available: u64 },

    /// Invalid amount
    #[error("Invalid amount: {0}")]
    InvalidAmount(String),

    /// Invalid duration
    #[error("Invalid duration: {0}")]
    InvalidDuration(String),

    /// Invalid nonce
    #[error("Invalid nonce: {0}")]
    InvalidNonce(String),

    /// PDA derivation error
    #[error("Failed to derive PDA: {0}")]
    PdaDerivationError(String),
}

impl From<std::io::Error> for BonkStakingError {
    fn from(err: std::io::Error) -> Self {
        BonkStakingError::SerializationError(err)
    }
}