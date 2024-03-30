mod de;
mod display;

use std::{collections::HashMap, fs};

use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::{error::TorrentError, torrent::de::*};

#[derive(Debug, Deserialize)]
pub struct Torrent {
    /// Announcer URL
    pub announce: String,
    /// Torrent information
    pub info: TorrentInfo,
    /// Non official fields
    #[serde(flatten)]
    pub additional_fields: RootAdditionalFields,
}

/// TorrentInfo is a struct that contains all the information about the torrent file.
#[derive(Debug, Deserialize)]
pub struct TorrentInfo {
    /// Recommanded output file or root directory
    pub name: String,
    /// Size of each data piece
    #[serde(rename = "piece length")]
    pub piece_length: u64,
    /// SHA1 hashes of each pieces
    #[serde(deserialize_with = "from_bytes_to_vec")]
    pub pieces: Vec<String>,
    /// In case of a single file, represents the file size
    pub length: Option<u64>,
    #[serde(default, deserialize_with = "from_files_list_to_struct")]
    /// In case of multiple files/directories, represents all files/directories available
    pub files: Option<Vec<TorrentFiles>>,
    // Additional fields available that are not part of the original specification
    #[serde(flatten)]
    pub additional_fields: TorrentInfoAdditionalFields,
}

#[derive(Debug, Deserialize)]
pub struct TorrentFiles {
    /// Output file path
    pub path: String,
    /// File size
    pub length: u64,
}

/// RootAdditionalFields is a struct that contains all the additional fields that are not part of the
/// original [BitTorrent](https://www.bittorrent.org/beps/bep_0003.html) specification.
#[derive(Debug, Deserialize)]
pub struct RootAdditionalFields {
    /// Torrent creator or software name
    #[serde(rename = "created by")]
    pub created_by: Option<String>,
    /// Torrent creation date
    #[serde(
        default,
        rename = "creation date",
        deserialize_with = "from_i64_to_datetime"
    )]
    pub creation_date: Option<DateTime<Utc>>,
    /// Comment about the torrent
    pub comment: Option<String>,
    // #[serde(rename = "url-list")]
    // /// List of resources available
    // pub url_list: Option<Vec<String>>,
    #[serde(flatten)]
    /// Extra fields not explicitly covered by the struct
    pub extra_fields: HashMap<String, serde_bencode::value::Value>,
}

/// TorrentInfoAdditionalFields is a struct that contains all the additional fields that are not part of the
/// original [BitTorrent](https://www.bittorrent.org/beps/bep_0003.html) specification.
#[derive(Debug, Deserialize)]
pub struct TorrentInfoAdditionalFields {
    /// Is the torrent private
    #[serde(default, deserialize_with = "from_bool_to_int")]
    pub private: bool,
    /// Extra fields not explicitly covered by the struct
    #[serde(flatten)]
    pub extra_fields: HashMap<String, serde_bencode::value::Value>,
}

pub fn parse(path: String) -> Result<Torrent, TorrentError> {
    let torrent_file = fs::read(path).map_err(|e| TorrentError::ReadTorrent(e))?;

    serde_bencode::from_bytes(&torrent_file).map_err(|e| TorrentError::ParseTorrent(e.to_string()))
}
