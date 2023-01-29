// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use lopdf::Document;
use std::path::Path;

use crate::error::{Error, ErrorKind};

pub struct PdfReader {
    doc: Document,
}

impl PdfReader {
    /// Open a pdf file.
    ///
    /// # Errors
    /// Returns error if this pdf file is not found or invalid.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let doc = Document::load(path)?;
        if doc.is_encrypted() {
            return Err(Error::from_string(
                ErrorKind::InvalidPdfFile,
                "password is missing".to_string(),
            ));
        }
        Ok(Self { doc })
    }

    #[must_use]
    pub fn pages(&self) -> u32 {
        let mut page_counter = 0;
        for _page in self.doc.page_iter() {
            page_counter += 1;
        }
        page_counter
    }
}
