use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum TorrentError {
    #[error("Failed to parse torrent file: {0}")]
    ParseTorrent(bendy::serde::Error),
    #[error("Failed to encode info dictionnary: {0}")]
    EncodeInfo(bendy::serde::Error),
    #[error("Failed to read torrent file: {0}")]
    ReadTorrent(#[from] io::Error),
}

#[derive(Error, Debug)]
pub enum TcpError {

}
