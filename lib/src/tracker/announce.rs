use std::collections::HashMap;

use bendy::{serde::from_bytes, value::Value};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, BoolFromInt};

use super::{errors::TrackerError, parsing_modules, Peer, Tracker};

#[derive(Debug, Clone)]
pub struct AnnounceRsp<'a> {
    pub interval: u64,
    pub peers: Vec<Peer>,
    pub additional_fields: HashMap<String, Value<'a>>,
}

/// Possible events sent when doing the announce request
#[derive(Default, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AnnounceEv {
    Started,
    Completed,
    Stopped,
    #[default]
    Empty,
}

/// Tracker query parameters.
#[serde_as]
#[derive(Default, Serialize)]
pub struct AnnounceReq {
    /// SHA1 hash of the bencode form. Must be 20 bytes long.
    /// REQUIRED
    #[serde(serialize_with = "parsing_modules::serialize_bytes_urlencoded")]
    pub info_hash: Vec<u8>,
    /// 20 characters ID generate before a download request.
    /// REQUIRED
    pub peer_id: String,
    /// Client's IP address.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub ip: String,
    /// Client's listening port.
    /// Usually, downloader will try common range: `6881` to `6889`.
    /// REQUIRED
    pub port: u16,
    /// Total amount of bytes uploaded encoded in base 10 `ASCII`.
    /// REQUIRED
    pub uploaded: String,
    /// Total amount of bytes downloaded encoded in base 10 `ASCII`.
    /// REQUIRED
    pub downloaded: String,
    /// Total amount of bytes left to download encoded in base 10 `ASCII`.
    /// REQUIRED
    pub left: String,
    /// Annoucement event.
    #[serde(default)]
    pub event: AnnounceEv,
    /// Should the tracker respond with a compact peers list
    #[serde_as(as = "BoolFromInt")]
    #[serde(default)]
    pub compact: bool,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Body<'a> {
    /// Tracker responded with an error
    Error {
        /// REQUIRED
        #[serde(rename = "failure reason")]
        failure_reason: String,
    },
    /// Tracker responded with a peer list - Normal response
    Success {
        /// Interval in seconds to query the tracker
        /// REQUIRED
        interval: u64,
        /// List of peers
        /// REQUIRED
        peers: Vec<Peer>,
    },
    /// Tracker responded with a peer list - Compact response
    SuccessCompact {
        /// Interval in seconds to query the tracker
        /// REQUIRED
        interval: u64,
        /// List of peers in BigEndian order.
        /// 4 bytes allocated for the IPv4 address and 2 bytes for the port.
        /// REQUIRED - if `peers6` key is not present. Both can be present
        #[serde(default, deserialize_with = "parsing_modules::deserialize_ipv4")]
        peers: Vec<Peer>,
        /// List of peers in BigEndian order.
        /// 16 bytes allocated for the IPv4 address and 2 bytes for the port.
        /// REQUIRED - if `peers` key is not present. Both can be present
        #[serde(default, deserialize_with = "parsing_modules::deserialize_ipv6")]
        peers6: Vec<Peer>,
        #[serde(flatten, borrow)]
        additional_fields: HashMap<String, Value<'a>>,
    },
}

impl Tracker {
    /// Trigger an annouce request to the torrent's tracker.
    /// Response will contain a peer list.
    pub async fn announce(&mut self, req: AnnounceReq) -> Result<Vec<u8>, TrackerError> {
        let req = reqwest::Client::new()
            .get(&self.url)
            .query(&req)
            .send()
            .await
            .unwrap();
        if !req.status().is_success() {
            return Err(TrackerError::InvalidStatus(req.status().as_u16()));
        }

        Ok(req.bytes().await?.to_vec())
    }

    pub async fn convert_bytes(&self, bytes: &'_ [u8]) -> Result<AnnounceRsp<'_>, TrackerError> {
        match from_bytes::<'_, Body>(bytes).map_err(|e| TrackerError::BencodeDecode(e))? {
            Body::Error { failure_reason } => {
                return Err(TrackerError::AnnounceFailed(failure_reason))
            }
            Body::Success { interval, peers } => Ok(AnnounceRsp {
                interval,
                peers,
                additional_fields: HashMap::new(),
            }),
            Body::SuccessCompact {
                interval,
                mut peers,
                peers6,
                additional_fields,
            } => {
                peers.extend(peers6);
                Ok(AnnounceRsp {
                    interval,
                    peers,
                    additional_fields: additional_fields
                        .into_iter()
                        .map(|(k, v)| (k, v.into_owned()))
                        .collect(),
                })
            }
        }
    }
}
