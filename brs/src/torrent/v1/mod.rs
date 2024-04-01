mod display;
mod ext_parsing;
mod main;

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, BoolFromInt, TimestampSeconds};

#[derive(Debug, Deserialize, Serialize)]
pub struct Torrent<'a> {
    /// Announcer URL
    pub announce: String,
    /// Torrent information
    pub info: TorrentInfo<'a>,
    /// Non official fields
    #[serde(flatten, borrow)]
    pub additional_fields: RootAdditionalFields<'a>,
}

/// TorrentInfo is a struct that contains all the information about the torrent file.
#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
pub struct TorrentInfo<'a> {
    /// Recommanded output file or root directory.
    /// REQUIRED
    pub name: String,
    /// Size of each data piece.
    /// REQUIRED
    #[serde(rename = "piece length")]
    pub piece_length: i64,
    /// SHA1 hashes of each pieces concatenated. Each hash is 20 bytes long.
    /// REQUIRED
    #[serde(with = "ext_parsing::pieces")]
    pub pieces: Vec<String>,
    /// In case of a single file, represents the file size.
    /// REQUIRED - If `TorrentInfo.files` is empty
    #[serde(default, skip_serializing_if = "ext_parsing::skip_empty::i64")]
    pub length: i64,
    /// In case of multiple files/directories, represents all files/directories available
    /// REQUIRED - If `TorrentInfo.length` is empty
    #[serde(
        default,
        with = "ext_parsing::files",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub files: Vec<TorrentFile>,
    // Additional fields available that are not part of the original specification
    #[serde(flatten, borrow)]
    pub additional_fields: TorrentInfoAdditionalFields<'a>,
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct TorrentFile {
    /// Output file path
    /// REQUIRED
    pub path: String,
    /// File size
    /// REQUIRED
    pub length: i64,
}

/// RootAdditionalFields contains all the additional fields that are not part of the
/// original [BitTorrent](https://www.bittorrent.org/beps/bep_0003.html) specification.
/// Those who are well known are mapped directly with default values.
#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
pub struct RootAdditionalFields<'a> {
    /// Torrent creator or software name
    #[serde(
        default,
        rename = "created by",
        skip_serializing_if = "ext_parsing::skip_empty::string"
    )]
    pub created_by: String,
    /// Torrent creation date
    #[serde_as(as = "TimestampSeconds<i64>")]
    #[serde(
        default,
        rename = "creation date",
        skip_serializing_if = "ext_parsing::skip_empty::date"
    )]
    pub creation_date: DateTime<Utc>,
    /// Comment about the torrent
    #[serde(default, skip_serializing_if = "ext_parsing::skip_empty::string")]
    pub comment: String,
    /// List of resources available
    #[serde(default, rename = "url-list", skip_serializing_if = "Vec::is_empty")]
    pub url_list: Vec<String>,
    #[serde(default)]
    pub encoding: String,
    /// Extra fields not explicitly covered by the struct
    #[serde(flatten, borrow)]
    pub extra_fields: HashMap<String, bendy::value::Value<'a>>,
}

/// TorrentInfoAdditionalFields contains all the additional fields that are not part of the
/// original [BitTorrent](https://www.bittorrent.org/beps/bep_0003.html) specification.
/// Those who are well known are mapped directly with default values.
#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
pub struct TorrentInfoAdditionalFields<'a> {
    /// Is the torrent private
    #[serde_as(as = "BoolFromInt")]
    #[serde(default, skip_serializing_if = "ext_parsing::skip_empty::bool")]
    pub private: bool,
    /// Extra fields not explicitly covered by the struct
    #[serde(flatten, borrow)]
    pub extra_fields: HashMap<String, bendy::value::Value<'a>>,
}
