use std::io;
use std::env;

use thiserror::Error;

/// The main error-handling type.
#[derive(Debug, Error)]
pub enum Error {
    /// An unexpected status code was received.
    #[error("unexpected status from GCP: {0}")]
    Status(#[from] tonic::Status),
    /// An error with the gRPC transport channel.
    #[error("transport error: {0}")]
    Transport(#[from] tonic::transport::Error),
    /// An IO error.
    #[error("IO error: {0}")]
    IO(#[from] io::Error),
    /// A JSON (de)serialization error.
    #[error("JSON error: {0}")]
    JSON(#[from] json::Error),
    /// An environment-related error (missing variable).
    #[error("environment error: {0}")]
    Env(#[from] env::VarError),
}