use std::fs;

use brs::{
    torrent::v1,
    tracker::{Tracker, TrackerRequest},
};
use rand::distributions::Alphanumeric;
use rand::Rng;

pub(crate) async fn check(path: String) {
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
        .announce(TrackerRequest {
            peer_id: format!("-BRS010-{peer_id}"),
            downloaded: "0".to_string(),
            left: torrent.calc_download_lenght().to_string(),
            uploaded: "0".to_string(),
            info_hash,
            compact: true,
            ..Default::default()
        })
        .await;

    if let Err(e) = rsp {
        eprintln!("{e}")
    }
}
