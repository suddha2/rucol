use thiserror::Error;

#[derive(Error, Debug)]
pub enum RucolError {
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("Ledger mismatch: {0}")]
    LedgerError(String),
    #[error("Unknown error")]
    Unknown,
}


#[derive(Debug, Error)]
pub enum LedgerError {
    #[error("Transaction is not balanced")]
    UnbalancedTransaction,
    #[error("Account not found")]
    AccountNotFound,
}

pub type Result<T> = std::result::Result<T, RucolError>;

