use crate::core::logger::LoggerEngine;
pub use thiserror::Error;

/// ============================================================================
/// 🚨 Centralized Error Engine (මධ්‍යගත දෝෂ පාලන එන්ජිම)
/// ============================================================================
/// සියලුම දෝෂ එක තැනකින් පාලනය වේ. ලොග් එන්ජිම සමඟ සම්බන්ධයි.

#[derive(Error, Debug, Clone)]
pub enum EngineError {
    #[error("ගණනය කිරීමේ දෝෂයකි: {message} (Code: {code})")]
    Calculation { code: String, message: String },

    #[error("වලංගු නොවන දත්ත: {message}")]
    Validation { message: String },

    #[error("පද්ධති දෝෂයකි: {message}")]
    System { message: String },

    #[error("ආරක්ෂක දෝෂයකි: {code} - {message}")]
    Security { code: String, message: String },

    #[error("සම්පත සොයා ගත නොහැක: {resource} (ID: {id})")]
    NotFound { resource: String, id: String },

    #[error("ගබඩා දෝෂයකි: {message}")]
    Storage { message: String },

    #[error("ජාලක දෝෂයකි: {message}")]
    Network { message: String },

    #[error("අවසරය ප්‍රතික්ෂේප විය: {message}")]
    Unauthorized { message: String },

    #[error("සීමාව ඉක්මවා ඇත: {message}")]
    RateLimited { message: String },

    #[error("ගනුදෙනු දෝෂයකි: {message}")]
    Transaction { message: String },

    #[error("ලෙජර් සමතුලිත නැත: Debit={debit}, Credit={credit}")]
    LedgerImbalance { debit: i64, credit: i64 },

    #[error("බාහිර සේවා දෝෂයකි: {service} - {message}")]
    ExternalService { service: String, message: String },

    #[error("දත්ත සමුදා දෝෂයකි: {message}")]
    Database { message: String },
}

pub type EngineResult<T> = Result<T, EngineError>;

pub struct ErrorHandler;

impl ErrorHandler {
    /// දෝෂයක් වාර්තා කරන්න (Report and Log Error)
    pub fn report(err: EngineError) -> EngineError {
        // Log the error automatically when reported
        let msg = format!("{:?}", err);
        LoggerEngine::error(&msg);
        err
    }
}
