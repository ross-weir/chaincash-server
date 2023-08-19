//! ChainCash server Error module.
use thiserror::Error;

/// Represents errors that can occur in the ChainCash server.
#[derive(Error, Debug)]
pub enum Error {
    /// An [`hyper::Error`] occurred, possibly during server creation.
    #[error("hyper error: {0}")]
    Hyper(#[from] hyper::Error),
}
