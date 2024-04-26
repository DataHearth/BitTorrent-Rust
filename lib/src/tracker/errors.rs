use thiserror::Error;

#[derive(Debug, Error)]
pub enum TrackerError {
    #[error("Failed to decode response body: {0}")]
    BencodeDecode(bendy::serde::Error),
    #[error("Tracker responded with an invalid status code: {0}")]
    InvalidStatus(u16),
    #[error("Announce request failed: {0}")]
    AnnounceFailed(String),
    #[error("Failed to get response body: {0}")]
    BodyDecode(#[from] reqwest::Error),
}
