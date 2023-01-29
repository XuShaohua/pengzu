// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use std::path::Path;

use crate::error::{Error, ErrorKind};
use crate::formats::mobi_reader::MobiReader;
use crate::parsers::cip::{is_plain_cip_page, parse_cip_from_html, CipRecord};

/// Parse CIP record from mobi file.
///
/// # Errors
/// Returns error if failed to parse mobi file.
pub fn parse_cip_from_mobi<P: AsRef<Path>>(path: P) -> Result<CipRecord, Error> {
    let reader = MobiReader::open(path)?;
    let pages = reader.pages();
    println!("pages: {pages}");

    // First 5 pages.
    let mut front_page = 0;
    while front_page < 5 && front_page < pages {
        let text = reader.read_page(front_page)?;
        println!("front page: {front_page}, text: {text}");
        if is_plain_cip_page(&text) {
            return parse_cip_from_html(&text);
        }
        front_page += 1;
    }

    // Last 5 pages.
    let mut rear_page = (pages - 5).max(front_page);
    while rear_page < pages {
        let text = reader.read_page(rear_page)?;
        println!("rear_page: {rear_page}, text: {text}");
        if is_plain_cip_page(&text) {
            return parse_cip_from_html(&text);
        }

        rear_page += 1;
    }

    Err(Error::new(
        ErrorKind::NoCipRecordFound,
        "No cip record found in epub file",
    ))
}
