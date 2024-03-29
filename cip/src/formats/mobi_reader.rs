// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use encoding_rs::WINDOWS_1252;
use mobi::headers::TextEncoding;
use mobi::Mobi;
use std::path::Path;

use crate::error::{Error, ErrorKind};

pub struct MobiReader {
    doc: Mobi,
}

impl MobiReader {
    /// Open an mobi file.
    ///
    /// # Errors
    /// Returns error if mobi file does not found or is invalid.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let doc = Mobi::from_path(path)?;
        Ok(Self { doc })
    }

    #[must_use]
    pub fn pages(&self) -> usize {
        self.doc.readable_records_range().len()
    }

    /// Read specific page.
    ///
    /// # Errors
    /// Returns error if failed to read file content.
    pub fn read_page(&self, page: usize) -> Result<String, Error> {
        let encoding = self.doc.text_encoding();
        let records = self.doc.raw_records();
        let records = records.records();
        if page < records.len() {
            let record = records[page];
            let content = record.content;
            match encoding {
                TextEncoding::UTF8 | TextEncoding::Unknown(_) => {
                    Ok(String::from_utf8_lossy(content).into_owned())
                }
                TextEncoding::CP1252 => match WINDOWS_1252.decode_with_bom_removal(content) {
                    (dest, true) => Ok(dest.to_string()),
                    (_, false) => Err(Error::new(ErrorKind::InvalidMobiPage, "Encode error")),
                },
            }
        } else {
            Err(Error::new(ErrorKind::InvalidMobiPage, "Invalid page index"))
        }
    }
}
