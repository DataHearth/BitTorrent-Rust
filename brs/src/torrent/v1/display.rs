use human_bytes::human_bytes;

use crate::{torrent::v1::ext_parsing::skip_empty, write_optional};

use super::Torrent;

impl std::fmt::Display for Torrent<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "GENERAL\n\n")?;
        write!(f, "  Tracker: {}\n", self.announce)?;
        write_optional!(
            f,
            "  Created by",
            &self.additional_fields.created_by,
            skip_empty::string
        );
        write_optional!(
            f,
            "  Creation date",
            &self.additional_fields.creation_date,
            skip_empty::date
        );
        write_optional!(
            f,
            "  Comment",
            &self.additional_fields.comment,
            skip_empty::string
        );
        write_optional!(
            f,
            "  Encoding",
            &self.additional_fields.encoding,
            skip_empty::string
        );
        if !self.additional_fields.url_list.is_empty() {
            write!(f, "  Additional resources:\n")?;
            for ar in &self.additional_fields.url_list {
                write!(f, "    - {}\n", ar)?;
            }
        }
        if !self.additional_fields.extra_fields.is_empty() {
            for (k, v) in &self.additional_fields.extra_fields {
                write!(f, "  {}: {:#?}\n", k, v,)?;
            }
        }
        write!(f, "\n")?;

        write!(f, "TORRENT INFORMATION\n\n")?;
        write!(f, "  Name: {}\n", self.info.name)?;
        write!(f, "  Hash: {}\n", self.calc_hash().unwrap())?;
        write_optional!(
            f,
            "  Private",
            &self.info.additional_fields.private,
            skip_empty::bool
        );
        write!(f, "  Pieces: {:?}\n", self.info.pieces.len())?;
        write!(
            f,
            "  Piece size: {}\n",
            human_bytes(self.info.piece_length as f64)
        )?;
        write!(
            f,
            "  Total size: {}\n",
            human_bytes(self.calc_download_lenght() as f64)
        )?;
        if !self.info.additional_fields.extra_fields.is_empty() {
            for (k, v) in &self.info.additional_fields.extra_fields {
                write!(f, "  {}: {:#?}\n", k, v,)?;
            }
        }

        write!(f, "\nFILES\n\n")?;
        if !self.info.files.is_empty() {
            for files in &self.info.files {
                write!(
                    f,
                    "  {} ({})\n",
                    files.path,
                    human_bytes(files.length as f64)
                )?;
            }
        } else {
            write!(
                f,
                "  {} ({})",
                self.info.name,
                human_bytes(self.calc_download_lenght() as f64)
            )?;
        }

        Ok(())
    }
}
