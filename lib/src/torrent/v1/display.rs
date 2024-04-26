use human_bytes::human_bytes;

use crate::{extension_parsing, write_optional};

use super::Torrent;

impl std::fmt::Display for Torrent<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "GENERAL\n")?;
        writeln!(f, "  Tracker: {}", self.announce)?;
        if !self.announce_list.is_empty() {
            writeln!(f, "  Additional trackers:")?;
            for i in &self.announce_list {
                writeln!(f, "  - {i}")?;
            }
        }
        write_optional!(
            f,
            "  Created by",
            &self.additional_fields.created_by,
            String::is_empty
        );
        write_optional!(
            f,
            "  Creation date",
            &self.additional_fields.creation_date,
            extension_parsing::skip_empty::date
        );
        write_optional!(
            f,
            "  Comment",
            &self.additional_fields.comment,
            String::is_empty
        );
        write_optional!(
            f,
            "  Encoding",
            &self.additional_fields.encoding,
            String::is_empty
        );
        if !self.additional_fields.url_list.is_empty() {
            writeln!(f, "  Additional resources:")?;
            for ar in &self.additional_fields.url_list {
                writeln!(f, "    - {}", ar)?;
            }
        }
        if !self.additional_fields.extra_fields.is_empty() {
            for (k, v) in &self.additional_fields.extra_fields {
                writeln!(f, "  {}: {:#?}", k, v,)?;
            }
        }

        writeln!(f, "\nTORRENT INFORMATION\n")?;
        writeln!(f, "  Name: {}", self.info.name)?;
        let hash = match self.calc_hash() {
            Ok(v) => hex::encode(v),
            Err(e) => format!("Failed to calculate hash for torrent: {e}"),
        };
        writeln!(f, "  Hash: {hash}",)?;
        write_optional!(
            f,
            "  Private",
            &self.info.additional_fields.private,
            extension_parsing::skip_empty::bool
        );
        writeln!(f, "  Pieces: {:?}", self.info.pieces.len())?;
        writeln!(
            f,
            "  Piece size: {}",
            human_bytes(self.info.piece_length as f64)
        )?;
        writeln!(
            f,
            "  Total size: {}",
            human_bytes(self.calc_download_lenght() as f64)
        )?;
        if !self.info.additional_fields.extra_fields.is_empty() {
            for (k, v) in &self.info.additional_fields.extra_fields {
                writeln!(f, "  {}: {:#?}", k, v,)?;
            }
        }

        writeln!(f, "\nFILES\n")?;
        if !self.info.files.is_empty() {
            for files in &self.info.files {
                writeln!(f, "  {} ({})", files.path, human_bytes(files.length as f64))?;
            }
        } else {
            writeln!(
                f,
                "  {} ({})",
                self.info.name,
                human_bytes(self.calc_download_lenght() as f64)
            )?;
        }

        Ok(())
    }
}
