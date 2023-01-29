// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use std::fs::File;
use std::path::Path;

use crate::error::Error;

pub struct PdfReader {
    file: File,
}

impl PdfReader {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let file = File::open(path)?;
        Ok(Self { file })
    }
}
