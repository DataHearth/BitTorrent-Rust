pub mod announce;
mod errors;
mod parsing_modules;
pub mod tracker;

use std::net::IpAddr;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Peer {
    /// Unique identifier for the peer.
    /// It can be optional in case the tracker's response is in a compact form.
    #[serde(rename = "peer id")]
    pub id: Option<String>,
    /// Peer IP address. IPv4 or IPv6.
    // #[serde(deserialize_with = "parsing_modules::deserialize_ipaddr")]
    pub ip: IpAddr,
    /// Peer listening port.
    pub port: u16,
}

#[derive(Default, Debug)]
pub struct Tracker {
    /// Tracker URL
    pub url: String,
}
