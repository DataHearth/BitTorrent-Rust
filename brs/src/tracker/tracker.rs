use std::{collections::HashMap, net::IpAddr};

use bendy::serde::from_bytes;

use crate::error::TrackerError;

use super::{Tracker, TrackerPeer, TrackerRequest, TrackerResponse};

const IPV4_PEER_LEN: u8 = 6;
const IPV6_PEER_LEN: u8 = 18;

impl Tracker {
    /// Create a new instance of `Tracker`
    pub fn new(url: String) -> Self {
        Self {
            url,
            interval: None,
            peers: None,
        }
    }

    pub async fn announce(&mut self, req: TrackerRequest) -> Result<(), TrackerError> {
        let req = reqwest::Client::new()
            .get(&self.url)
            .query(&req)
            .send()
            .await?;
        if !req.status().is_success() {
            return Err(TrackerError::InvalidStatus(req.status().as_u16()));
        }

        let rsp: TrackerResponse = from_bytes(&req.bytes().await?)?;
        match rsp {
            TrackerResponse::Error { failure_reason } => {
                return Err(TrackerError::AnnounceFailed(failure_reason))
            }
            TrackerResponse::Success { interval, peers } => {
                let mut hashmap_peers = HashMap::new();
                for p in peers {
                    hashmap_peers.insert(
                        p.peer_id,
                        TrackerPeer {
                            ip: p.ip.parse()?,
                            port: p.port,
                        },
                    );
                }
                self.interval = Some(interval);
                self.peers = Some(hashmap_peers);
            }
            TrackerResponse::SuccessCompact {
                interval,
                peers,
                peers6,
            } => {
                let mut hashmap_peers = HashMap::new();
                if let Some(p) = peers {
                    if (p.len() % IPV4_PEER_LEN as usize) != 0 {
                        return Err(TrackerError::InvalidPeersCompactList(IPV4_PEER_LEN, p.len() as u64));
                    }
                    for (i, peer) in p.chunks(IPV4_PEER_LEN as usize).enumerate() {
                        let (ip, port) = peer.split_at(4);
                        hashmap_peers.insert(
                            i.to_string(),
                            TrackerPeer {
                                ip: IpAddr::from(TryInto::<[u8; 4]>::try_into(ip).expect(
                                    "cannot convert &[u8] to &[u8; 4] where chunks is already of lenght 4",
                                )),
                                port: u16::from_be_bytes(port.try_into().expect(
                                    "cannot convert &[u8] to &[u8; 2] where chunks is already of lenght 2",
                                )),
                            },
                        );
                    }
                }
                if let Some(p6) = peers6 {
                    if (p6.len() % IPV6_PEER_LEN as usize) != 0 {
                        return Err(TrackerError::InvalidPeersCompactList(IPV6_PEER_LEN, p6.len() as u64));
                    }
                    for (i, peer) in p6.chunks(IPV6_PEER_LEN as usize).enumerate() {
                        let (ip, port) = peer.split_at(14);
                        hashmap_peers.insert(
                            i.to_string(),
                            TrackerPeer {
                                ip: IpAddr::from(TryInto::<[u8; 16]>::try_into(ip).expect(
                                    "cannot convert &[u8] to &[u8; 16] where chunks is already of lenght 16",
                                )),
                                port: u16::from_be_bytes(port.try_into().expect(
                                    "cannot convert &[u8] to &[u8; 2] where chunks is already of lenght 2",
                                )),
                            },
                        );
                    }
                }
                self.interval = Some(interval);
                self.peers = Some(hashmap_peers);
            }
        }

        Ok(())
    }
}
