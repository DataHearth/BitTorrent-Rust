use bendy::serde::{from_bytes, to_bytes};
use sha1::{Digest, Sha1};

use crate::torrent::errors::TorrentError;

use super::{ext_parsing, Torrent};

impl Torrent<'_> {
    pub fn parse_bytes<'a>(bytes: &'a [u8]) -> Result<Torrent<'_>, TorrentError> {
        from_bytes::<'a>(bytes).map_err(|e| TorrentError::ParseTorrent(e))
    }

    pub fn calc_download_lenght(&self) -> i64 {
        if !ext_parsing::skip_empty::i64(&self.info.length) {
            return self.info.length;
        }

        self.info.files.iter().map(|f| f.length).sum()
    }

    pub fn calc_hash(&self) -> Result<String, TorrentError> {
        let mut hasher = Sha1::new();
        let encoded = to_bytes(&self.info).map_err(|e| TorrentError::EncodeInfo(e))?;
        hasher.update(&encoded);

        Ok(hex::encode(hasher.finalize()))
    }
}
