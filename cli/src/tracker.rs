use std::fs;

use brs::{
    torrent::v1,
    tracker::{announce::AnnounceReq, Tracker},
};
use rand::distributions::Alphanumeric;
use rand::Rng;

pub(crate) async fn peers(path: String) {
    let bytes = fs::read(path).unwrap();
    let torrent = v1::Torrent::parse_bytes(&bytes);
    if let Err(e) = &torrent {
        eprintln!("Failed to parse torrent: {e}")
    }

    let torrent = torrent.unwrap();
    let peer_id: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(12)
        .map(char::from)
        .collect();
    let info_hash = match torrent.calc_hash() {
        Ok(v) => v,
        Err(e) => return eprintln!("Failed to calculate info hash: {e}"),
    };
    let mut tracker = Tracker::new(torrent.announce.clone());
    let rsp = tracker
        .announce(AnnounceReq {
            peer_id: format!("-BR010-{peer_id}"),
            downloaded: "0".to_string(),
            left: torrent.calc_download_lenght().to_string(),
            uploaded: "0".to_string(),
            info_hash,
            compact: true,
            ..Default::default()
        })
        .await
        .unwrap();
    let rsp = tracker.convert_bytes(&rsp).await;

    match rsp {
        Ok(v) => {
            for p in v.peers {
                println!("- ip: {}", p.ip);
                println!("  port: {}", p.port);
                if let Some(id) = p.id {
                    println!("  id: {}", id);
                }
                println!("");
            }
        }
        Err(e) => eprintln!("Failed to get peers: {e}"),
    }
}
