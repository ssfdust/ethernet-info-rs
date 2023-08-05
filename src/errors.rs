//! Error library for ethtool.
//! All the errors that can be returned by this crate are defined here.

use std::error::Error;
use std::fmt;

/// The error type for this crate.
#[derive(Debug)]
pub struct EthtoolError {
    details: String,
}

impl EthtoolError {
    pub fn new(msg: &str) -> EthtoolError {
        EthtoolError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for EthtoolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for EthtoolError {}
