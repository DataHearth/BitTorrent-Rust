use std::net::AddrParseError;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum TrackerError {
    #[error("Failed to execute announce request: {0}")]
    AnnounceRequest(#[from] reqwest::Error),
    #[error("Failed to decode response body: {0}")]
    BencodeDecode(#[from] bendy::serde::Error),
    #[error("Tracker responded with an invalid status code: {0}")]
    InvalidStatus(u16),
    #[error("Announce request failed: {0}")]
    AnnounceFailed(String),
    #[error("Failed to convert IP string to IpAddr: {0}")]
    IpParse(#[from] AddrParseError),
    #[error("Invalid compact peers list. Expected a list of {0}*n bytes, found: {1}")]
    InvalidPeersCompactList(u8, u64),
    #[error("Failed to parse tracker URL: {0}")]
    ParseURL(String),
}
