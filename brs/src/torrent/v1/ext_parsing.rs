pub(super) mod skip_empty {
    use chrono::{DateTime, Utc};

    #[inline(always)]
    pub(crate) fn i64(v: &i64) -> bool {
        *v == 0
    }

    #[inline(always)]
    pub(crate) fn bool(v: &bool) -> bool {
        *v == false
    }

    #[inline(always)]
    pub(crate) fn string(v: &String) -> bool {
        v.len() == 0
    }

    #[inline(always)]
    pub(crate) fn date(v: &DateTime<Utc>) -> bool {
        *v == DateTime::<Utc>::default()
    }
}

pub(super) mod pieces {
    use serde::{de, ser, Deserializer, Serializer};
    use serde_with::{Bytes, DeserializeAs, SerializeAs};

    pub fn serialize<S>(pieces: &Vec<String>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut buf: Vec<u8> = vec![];
        for v in pieces {
            buf.append(&mut hex::decode(v).map_err(|e| {
                ser::Error::custom(format!(
                    "Every pieces must be serializable into hexadecimal: {e}"
                ))
            })?);
        }

        Bytes::serialize_as(&buf, serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bytes: Vec<u8> = Bytes::deserialize_as(deserializer)?;
        if (bytes.len() % 20) != 0 {
            return Err(de::Error::custom("Invalid SHA1 pieces"));
        }

        Ok(bytes.chunks(20).map(|c| hex::encode(c)).collect())
    }
}

pub(super) mod files {
    use std::{borrow::Cow, collections::BTreeMap};

    use bendy::value::Value;
    use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

    use crate::torrent::v1::TorrentFile;

    pub fn serialize<S>(files: &Vec<TorrentFile>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut dict_list = vec![];
        let length_key: Cow<'_, [u8]> = Cow::Owned(String::from("length").into_bytes());
        let path_key: Cow<'_, [u8]> = Cow::Owned(String::from("path").into_bytes());
        for f in files {
            let mut dict: BTreeMap<Cow<'_, [u8]>, Value> = BTreeMap::new();
            dict.insert(length_key.clone(), Value::Integer(f.length as i64));
            let mut path_partial = vec![];
            for i in f.path.split("/") {
                path_partial.push(Value::Bytes(i.as_bytes().into()));
            }
            dict.insert(path_key.clone(), Value::List(path_partial));
            dict_list.push(Value::Dict(dict));
        }

        dict_list.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<TorrentFile>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let values = match Value::deserialize(deserializer)? {
            Value::List(v) => v,
            Value::Bytes(v) => {
                return Err(de::Error::invalid_type(
                    de::Unexpected::Bytes(&v),
                    &"list of dict",
                ))
            }
            Value::Dict(_) => {
                return Err(de::Error::invalid_type(
                    de::Unexpected::Map,
                    &"list of dict",
                ))
            }
            Value::Integer(v) => {
                return Err(de::Error::invalid_type(
                    de::Unexpected::Signed(v),
                    &"list of dict",
                ))
            }
        };
        let mut torrent_files: Vec<TorrentFile> = vec![];
        let length_key = Cow::Owned(String::from("length").into_bytes());
        let path_key = Cow::Owned(String::from("path").into_bytes());
        for v in values {
            let file = match v {
                Value::Dict(v) => v,
                Value::Bytes(v) => {
                    return Err(de::Error::invalid_type(
                        de::Unexpected::Bytes(&v),
                        &"dict with keys length and path",
                    ))
                }
                Value::Integer(v) => {
                    return Err(de::Error::invalid_type(
                        de::Unexpected::Signed(v),
                        &"dict with keys length and path",
                    ))
                }
                Value::List(_) => {
                    return Err(de::Error::invalid_type(
                        de::Unexpected::Seq,
                        &"dict with keys length and path",
                    ))
                }
            };
            let length = file.get(&length_key);
            let path = file.get(&path_key);
            if length.is_none() {
                return Err(de::Error::missing_field("length"));
            }
            if path.is_none() {
                return Err(de::Error::missing_field("path"));
            }
            let length = match length.unwrap() {
                Value::Integer(v) => *v,
                Value::Bytes(v) => {
                    return Err(de::Error::invalid_type(
                        de::Unexpected::Bytes(&v),
                        &"integer",
                    ))
                }
                Value::Dict(_) => {
                    return Err(de::Error::invalid_type(de::Unexpected::Map, &"integer"))
                }
                Value::List(_) => {
                    return Err(de::Error::invalid_type(de::Unexpected::Seq, &"integer"))
                }
            };
            let path = match path.unwrap() {
                Value::List(v) => {
                    let mut final_str = String::new();
                    for (i, item) in v.iter().enumerate() {
                        let partial = match item {
                            Value::Bytes(v) => std::str::from_utf8(v).map_err(|_| {
                                de::Error::invalid_value(
                                    de::Unexpected::Bytes(v),
                                    &"valid UTF-8 string",
                                )
                            })?,
                            Value::Dict(_) => {
                                return Err(de::Error::invalid_type(de::Unexpected::Map, &"string"))
                            }
                            Value::Integer(v) => {
                                return Err(de::Error::invalid_type(
                                    de::Unexpected::Signed(*v),
                                    &"string",
                                ))
                            }
                            Value::List(_) => {
                                return Err(de::Error::invalid_type(de::Unexpected::Seq, &"string"))
                            }
                        };
                        if i == 0 {
                            final_str.push_str(partial);
                            continue;
                        }
                        final_str.push_str(&format!("/{partial}"));
                    }
                    final_str
                }
                Value::Bytes(v) => {
                    return Err(de::Error::invalid_type(
                        de::Unexpected::Bytes(&v),
                        &"list of bytes",
                    ))
                }
                Value::Dict(_) => {
                    return Err(de::Error::invalid_type(de::Unexpected::Map, &"list of bytes"))
                }
                Value::Integer(v) => {
                    return Err(de::Error::invalid_type(
                        de::Unexpected::Signed(*v),
                        &"list of bytes",
                    ))
                }
            };
            torrent_files.push(TorrentFile { length, path })
        }

        Ok(torrent_files)
    }
}
