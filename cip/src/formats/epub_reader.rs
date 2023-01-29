// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use epub::doc::EpubDoc;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::error::{Error, ErrorKind};

pub struct EpubReader {
    doc: EpubDoc<BufReader<File>>,
}

impl EpubReader {
    /// Open an epub file.
    ///
    /// # Errors
    /// Returns error if epub file does not found or is invalid.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let doc = EpubDoc::new(path)?;
        Ok(Self { doc })
    }

    #[must_use]
    pub fn pages(&self) -> usize {
        self.doc.get_num_pages()
    }

    /// Read text content of specific page.
    ///
    /// # Errors
    /// Returns error if failed to read page text.
    pub fn read_page(&self, page: usize) -> Result<String, Error> {
        if page < self.doc.spine.len() {
            Ok(self.doc.spine[page].clone())
        } else {
            Err(Error::from_string(
                ErrorKind::InvalidEpubPage,
                "Invalid page number".to_string(),
            ))
        }
    }
}
