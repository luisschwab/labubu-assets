//! Labubu Assets
//!
//! Error

use thiserror::Error;

/// Errors re. transaction building, network and image conversion.
#[derive(Debug, Error)]
pub(crate) enum LabubuError {
    #[error("Rounding error")]
    Rounding,

    #[error("Invalid network")]
    InvalidNetwork,

    #[error("Esplora error: {0}")]
    Esplora(#[from] esplora_client::Error),

    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Serde error: {0}")]
    Serde(#[from] serde_json::Error),
}
