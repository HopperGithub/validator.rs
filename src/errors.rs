//! # errors
// This module includes error enum and utility fuctions.
use slog::Logger;
use serde_json::Value;

/// Error types which can occur on database aperation.
#[derive(Clone, Debug)]
pub enum Errors {
    /// Record is boolean
    IsBoolean(String),
    /// Record is email
    IsEmail(String),
}

/// Takes the error, logs it and wraps inside a Result
pub fn log_n_wrap(logger: &Logger, error: Errors) -> Result<Value, Errors> {
    error!(logger, "{:?}", error);
    return Err(error);
}