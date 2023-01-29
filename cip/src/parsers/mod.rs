// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

pub mod cip;
pub mod epub_parser;
pub mod mobi_parser;
pub mod pdf_parser;

use std::path::Path;

use crate::error::{Error, ErrorKind};
use crate::parsers::cip::CipRecord;
use crate::parsers::epub_parser::parse_cip_from_epub;
use crate::parsers::mobi_parser::parse_cip_from_mobi;
use crate::parsers::pdf_parser::parse_cip_from_pdf;

/// Parse cip record from ebook file.
///
/// # Errors
/// Returns error if failed to parse ebook format or cip record.
pub fn parse_ebook_file<P: AsRef<Path>>(path: P) -> Result<CipRecord, Error> {
    let path_ref: &Path = path.as_ref();
    let extension = path_ref.extension().unwrap_or_default();

    if extension == "pdf" {
        return parse_cip_from_pdf(path);
    }
    if extension == "epub" {
        return parse_cip_from_epub(path);
    }
    if extension == "azw" || extension == "azw3" || extension == "mobi" {
        return parse_cip_from_mobi(path);
    }

    Err(Error::from_string(
        ErrorKind::UnsupportedFile,
        format!("File not supported, {path_ref:?}"),
    ))
}
