use std::io;

use thiserror::Error;

pub mod cmd;
pub mod search;
pub mod walk;

#[derive(Debug, Error)]
pub enum GrepErrors {
    #[error("io error: {0}")]
    IOError(#[from] io::Error),

    #[error("command error: {0}")]
    CommandError(String),

    #[error("regex error: {0}")]
    RegexError(#[from] regex::Error),

    #[error("ignore error: {0}")]
    IgnoreError(#[from] ignore::Error),
}

pub type GrepResult<T> = Result<T, GrepErrors>;
