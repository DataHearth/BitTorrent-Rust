use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum TorrentError {
    #[error("Failed to parse torrent file: {0}")]
    ParseTorrent(String),
    #[error("Failed to read torrent file: {0}")]
    ReadTorrent(#[from] io::Error)
}
