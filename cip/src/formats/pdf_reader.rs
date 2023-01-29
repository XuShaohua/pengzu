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
    /// Returns error if this pdf file does not found or is invalid.
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

    /// Get number of pages in pdf file.
    #[must_use]
    pub fn pages(&self) -> u32 {
        let mut page_counter = 0;
        for _page in self.doc.page_iter() {
            page_counter += 1;
        }
        page_counter
    }

    /// Read text content of specific page.
    ///
    /// # Errors
    /// Returns error if failed to read page text.
    pub fn read_page(&self, page: u32) -> Result<String, Error> {
        self.doc.extract_text(&[page]).map_err(Into::into)
    }

    /// Read text content of multiple pages.
    ///
    /// # Errors
    /// Returns error if failed to read page text.
    pub fn read_pages(&self, pages: &[u32]) -> Result<String, Error> {
        self.doc.extract_text(pages).map_err(Into::into)
    }
}
