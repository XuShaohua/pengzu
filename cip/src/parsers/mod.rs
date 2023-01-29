// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

pub mod cip;
pub mod epub_parser;
pub mod mobi_parser;
pub mod pdf_parser;

use std::path::Path;

use crate::error::Error;
use crate::parsers::cip::CipRecord;

/// Parse cip record from ebook file.
///
/// # Errors
/// Returns error if failed to parse ebook format or cip record.
pub fn parse_ebook_file<P: AsRef<Path>>(_path: P) -> Result<CipRecord, Error> {
    todo!()
}
