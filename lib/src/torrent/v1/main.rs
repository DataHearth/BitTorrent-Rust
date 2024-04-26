use bendy::serde::{from_bytes, to_bytes};
use sha1::{Digest, Sha1};

use crate::{extension_parsing, torrent::errors::TorrentError};

use super::Torrent;

impl Torrent<'_> {
    pub fn parse_bytes<'a>(bytes: &'a [u8]) -> Result<Torrent<'_>, TorrentError> {
        from_bytes::<'_>(bytes).map_err(|e| TorrentError::ParseTorrent(e))
    }

    pub fn calc_download_lenght(&self) -> i64 {
        if !extension_parsing::skip_empty::i64(&self.info.length) {
            return self.info.length;
        }

        self.info.files.iter().map(|f| f.length).sum()
    }

    pub fn calc_hash(&self) -> Result<Vec<u8>, TorrentError> {
        let mut hasher = Sha1::new();
        let encoded = to_bytes(&self.info).map_err(|e| TorrentError::EncodeInfo(e))?;
        hasher.update(&encoded);
        let hash = hasher.finalize();

        Ok(hash.to_vec())
    }
}
