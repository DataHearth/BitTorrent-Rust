use std::collections::HashMap;

use human_bytes::human_bytes;

use crate::write_option;

use super::Torrent;

impl std::fmt::Display for Torrent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "tracker: {}\n", self.announce)?;
        write_option!(f, "created by", &self.additional_fields.created_by);
        write_option!(f, "creation date", &self.additional_fields.creation_date);
        write_option!(f, "comment", &self.additional_fields.comment);
        if self.additional_fields.extra_fields.len() > 0 {
            for (k, v) in self.additional_fields.extra_fields.clone().into_iter() {
                let value = match v {
                    serde_bencode::value::Value::Bytes(v) => std::str::from_utf8(&v)
                        .map_err(|_| std::fmt::Error)?
                        .to_string(),
                    serde_bencode::value::Value::Int(v) => v.to_string(),
                    serde_bencode::value::Value::List(v) => {
                        display_list(f, v)?;
                        continue;
                    }
                    serde_bencode::value::Value::Dict(v) => {
                        display_dict(f, v)?;
                        continue;
                    }
                };
                write!(f, "{k}: {value}\n")?;
            }
        }
        write!(f, "\n")?;

        write!(f, "Torrent information:\n")?;
        write!(f, "\tname: {}\n", self.info.name)?;
        write!(f, "\tprivate: {}\n", self.info.additional_fields.private)?;
        if let Some(v) = self.info.length {
            write!(f, "\tfile size: {}\n", human_bytes(v as f64))?;
        }
        write!(f, "\tpieces: {:?}\n", self.info.pieces.len())?;
        write!(
            f,
            "\tpiece size: {}\n",
            human_bytes(self.info.piece_length as f64)
        )?;
        if std::env::var("BRS_PRINT_TORRENT_FILES").is_ok() {
            if let Some(v) = &self.info.files {
                write!(f, "\tfiles:\n")?;
                for file in v {
                    write!(f, "\t  - {}\n", file.path)?;
                    write!(f, "\t  size: {}\n", human_bytes(file.length as f64))?;
                }
            }
        }
        if self.info.additional_fields.extra_fields.len() > 0 {
            for (k, v) in &self.info.additional_fields.extra_fields {
                write!(f, "\t{}: {:#?}\n", k, v)?;
            }
        }

        Ok(())
    }
}

fn display_dict(
    f: &mut std::fmt::Formatter,
    dict: HashMap<Vec<u8>, serde_bencode::value::Value>,
) -> std::fmt::Result {
    for (k, v) in dict {
        let key = std::str::from_utf8(&k).map_err(|_| std::fmt::Error)?;
        let value = match v {
            serde_bencode::value::Value::Bytes(v) => std::str::from_utf8(&v)
                .map_err(|_| std::fmt::Error)?
                .to_string(),
            serde_bencode::value::Value::Int(v) => v.to_string(),
            serde_bencode::value::Value::List(v) => {
                display_list(f, v)?;
                continue;
            }
            serde_bencode::value::Value::Dict(v) => {
                display_dict(f, v)?;
                continue;
            }
        };
        write!(f, "{key}: {value}\n").map_err(|_| std::fmt::Error)?
    }

    Ok(())
}

fn display_list(
    f: &mut std::fmt::Formatter,
    list: Vec<serde_bencode::value::Value>,
) -> std::fmt::Result {
    for element in list {
        let value = match element {
            serde_bencode::value::Value::Bytes(v) => std::str::from_utf8(&v)
                .map_err(|_| std::fmt::Error)?
                .to_string(),
            serde_bencode::value::Value::Int(v) => v.to_string(),
            serde_bencode::value::Value::List(v) => {
                display_list(f, v)?;
                continue;
            }
            serde_bencode::value::Value::Dict(v) => {
                display_dict(f, v)?;
                continue;
            }
        };
        write!(f, "  - {value}\n")?;
    }

    Ok(())
}
