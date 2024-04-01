//! Tracker operations
//!
//! Start by creating an instance of a tracker
//! ```rust
//! use brs::torrent::v1::Torrent;
//! use brs::torrent::Parse;
//! use brs::tracker::Tracker;
//!
//! fn main() {
//!     let torrent = match Torrent::parse("./file.torrent") {
//!         Ok(v) => v,
//!         Err(e) => return eprintln!("{e}"),
//!     };
//!
//!     let tracker = Tracker::new(&torrent.url);
//! }
//! ```

mod tracker;

use std::{collections::HashMap, net::IpAddr};

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none, BoolFromInt};

#[derive(Default, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TrackerEvent {
    Started,
    Completed,
    Stopped,
    #[default]
    Empty,
}

/// Tracker query parameters.
#[serde_as]
#[skip_serializing_none]
#[derive(Default, Serialize)]
pub struct TrackerRequest {
    /// SHA1 hash of the bencode form. Must be 20 bytes long.
    pub info_hash: String,
    /// 20 characters ID generate before a download request.
    pub peer_id: String,
    /// Client's IP address.
    pub ip: Option<IpAddr>,
    /// Client's listening port.
    /// Usually, downloader will try common range: `6881` to `6889`.
    pub port: Option<u16>,
    /// Total amount of bytes uploaded encoded in base 10 `ASCII`.
    pub uploaded: String,
    /// Total amount of bytes downloaded encoded in base 10 `ASCII`.
    pub downloaded: String,
    /// Total amount of bytes left to download encoded in base 10 `ASCII`.
    pub left: String,
    /// Annoucement event.
    pub event: TrackerEvent,
    /// Should the tracker respond with a compact peers list
    #[serde_as(as = "BoolFromInt")]
    pub compact: bool,
}

#[derive(Deserialize)]
pub enum TrackerResponse {
    /// Tracker responded with an error
    Error {
        #[serde(rename = "failure reason")]
        failure_reason: String,
    },
    /// Tracker successfully computed the query - Normal response
    Success {
        /// Interval in seconds to query the tracker
        interval: u64,
        /// List of peers
        peers: Vec<Peer>,
    },
    /// Tracker successfully computed the query - Compact response
    SuccessCompact {
        /// Interval in seconds to query the tracker
        interval: u64,
        /// List of peers in BigEndian order.
        /// 4 bytes allocated for the IPv4 address and 2 bytes for the port.
        peers: Option<Vec<u8>>,
        peers6: Option<Vec<u8>>
    },
}

#[derive(Deserialize)]
pub struct Peer {
    /// Unique identifier for the peer
    #[serde(rename = "peer id")]
    pub peer_id: String,
    /// Peer IP address. IPv4 or IPv6
    pub ip: String,
    /// Peer listening port
    pub port: u16,
}

pub struct Tracker {
    /// Tracker URL
    pub url: String,
    /// Interval in seconds to query the tracker once the transfert has started.
    /// /!\ Populated with the first announce query.
    pub interval: Option<u64>,
    /// List of peers.
    /// /!\ Populated with the first announce query.
    pub peers: Option<HashMap<String, TrackerPeer>>,
}

pub struct TrackerPeer {
    pub ip: IpAddr,
    pub port: u16,
}
