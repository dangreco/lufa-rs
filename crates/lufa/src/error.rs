use core::fmt;
use std::string::FromUtf8Error;

use snafu::{Backtrace, Snafu};

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Snafu, Debug)]
#[snafu(visibility(pub))]
pub enum Error {
    Lufa {
        source: LufaError,
        backtrace: Backtrace,
    },

    UrlEncoding {
        source: FromUtf8Error,
        backtrace: Backtrace,
    },

    SerdePhp {
        source: serde_php::Error,
        backtrace: Backtrace,
    },

    #[snafu(display("Reqwest Error: {}\nFound at {}", source, backtrace))]
    Reqwest {
        source: reqwest::Error,
        backtrace: Backtrace,
    },
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct LufaError {
    pub message: String,
}

impl fmt::Display for LufaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)?;
        Ok(())
    }
}

impl std::error::Error for LufaError {}
