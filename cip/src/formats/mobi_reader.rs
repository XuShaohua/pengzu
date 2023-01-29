// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use mobi::Mobi;
use std::path::Path;

use crate::error::Error;

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
    pub const fn pages(&self) -> usize {
        1
    }

    /// Read all readable contents.
    ///
    /// # Errors
    /// Returns error if failed to read file content.
    pub fn read_all(&self) -> Result<String, Error> {
        self.doc.content_as_string().map_err(Into::into)
    }
}
