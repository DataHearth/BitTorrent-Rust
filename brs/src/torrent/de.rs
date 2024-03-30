use chrono::{DateTime, Utc};
use serde::{
    de::{Error, Unexpected},
    Deserialize, Deserializer,
};

use crate::{match_bytes, match_dict, match_int, match_list, torrent::TorrentFiles};

pub fn from_bool_to_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match u8::deserialize(deserializer)? {
        0 => Ok(false),
        1 => Ok(true),
        other => Err(Error::invalid_value(
            Unexpected::Unsigned(other as u64),
            &"zero or one",
        )),
    }
}

pub fn from_i64_to_datetime<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let timestamp: Option<i64> = Option::deserialize(deserializer)?;
    if let Some(v) = timestamp {
        return Ok(DateTime::from_timestamp(v, 0));
    }

    Ok(None)
}

pub fn from_bytes_to_vec<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let bytes = serde_bytes::ByteBuf::deserialize(deserializer)?;
    Ok(bytes
        .chunks(20)
        .map(|v| hex::encode(v))
        .collect::<Vec<String>>())
}

pub fn from_files_list_to_struct<'de, D>(
    deserializer: D,
) -> Result<Option<Vec<TorrentFiles>>, D::Error>
where
    D: Deserializer<'de>,
{
    let list: Vec<serde_bencode::value::Value> = match Option::deserialize(deserializer)? {
        Some(v) => v,
        None => return Ok(None),
    };
    let mut torrent_files = vec![];
    for v in list {
        let file_dict = match_dict!(v, "map with keys \"path\" & \"length\"")?;
        if file_dict.len() > 2 {
            return Err(Error::invalid_length(
                file_dict.len(),
                &"path and length only",
            ));
        }
        let path = file_dict.get(b"path".as_ref());
        let length = file_dict.get(b"length".as_ref());
        if path.is_none() {
            return Err(Error::missing_field(
                "\"path\" is mandatory in a files list",
            ));
        } else if length.is_none() {
            return Err(Error::missing_field(
                "\"length\" is mandatory in a files list",
            ));
        }
        let path = {
            let mut str_path = String::new();
            for chunks in match_list!(path.unwrap(), "list of bytes")? {
                let chunks = match_bytes!(chunks, "sequence of bytes")?;
                str_path.push_str(std::str::from_utf8(chunks).map_err(|_| {
                    Error::invalid_value(Unexpected::Bytes(&chunks), &"Invalid bytes string")
                })?);
            }
            str_path
        };
        let length = match_int!(length.unwrap(), "integer")?;
        torrent_files.push(TorrentFiles {
            path,
            length: *length as u64,
        })
    }

    Ok(Some(torrent_files))
}
