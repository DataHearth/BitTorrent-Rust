use std::net::IpAddr;

use serde::{Deserializer, Serialize, Serializer};
use serde_with::{Bytes, DeserializeAs};

use super::Peer;

const PEER_IPV4_CHUNK_LEN: u8 = 6;
const PEER_IPV6_CHUNK_LEN: u8 = 18;

pub fn deserialize_ipv4<'de, D>(deserializer: D) -> Result<Vec<Peer>, D::Error>
where
    D: Deserializer<'de>,
{
    let bytes: &[u8] = Bytes::deserialize_as(deserializer)?;

    let mut peers = vec![];
    for c in bytes.chunks(PEER_IPV4_CHUNK_LEN as usize) {
        let (ip_c, port_c) = c.split_at(4);
        peers.push(Peer {
            id: None,
            ip: IpAddr::from(
                TryInto::<[u8; 4]>::try_into(ip_c).expect("ipv4 chunk should be of length 4"),
            ),
            port: u16::from_be_bytes(
                TryInto::<[u8; 2]>::try_into(port_c).expect("port chunk should be of length 2"),
            ),
        });
    }

    Ok(peers)
}

pub fn deserialize_ipv6<'de, D>(deserializer: D) -> Result<Vec<Peer>, D::Error>
where
    D: Deserializer<'de>,
{
    let bytes: &[u8] = Bytes::deserialize_as(deserializer)?;

    let mut peers = vec![];
    for c in bytes.chunks(PEER_IPV6_CHUNK_LEN as usize) {
        let (ip_c, port_c) = c.split_at(16);
        peers.push(Peer {
            id: None,
            ip: IpAddr::from(
                TryInto::<[u8; 16]>::try_into(ip_c).expect("ipv6 chunk should be of length 16"),
            ),
            port: u16::from_be_bytes(
                TryInto::<[u8; 2]>::try_into(port_c).expect("port chunk should be of length 2"),
            ),
        });
    }

    Ok(peers)
}

pub fn serialize_bytes_urlencoded<S>(bytes: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    unsafe { std::str::from_utf8_unchecked(bytes) }.serialize(serializer)
}
