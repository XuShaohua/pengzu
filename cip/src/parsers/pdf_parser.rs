// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use std::path::Path;

use crate::error::{Error, ErrorKind};
use crate::formats::pdf_reader::PdfReader;
use crate::parsers::cip::{is_plain_cip_page, parse_cip_from_text, CipRecord};

pub fn parse_cip_from_pdf<P: AsRef<Path>>(path: P) -> Result<CipRecord, Error> {
    let reader = PdfReader::open(path)?;
    let pages = reader.pages();

    // First 10 pages.
    let mut front_page = 1;
    while front_page < 10 && front_page < pages {
        let text = reader.read_page(front_page)?;
        println!("page: {front_page}, text: {text}");
        if is_plain_cip_page(&text) {
            return parse_cip_from_text(&text);
        }

        front_page += 1;
    }

    // Last 5 pages.
    let mut rear_page = (pages - 5).max(front_page);
    while rear_page < pages {
        let text = reader.read_page(rear_page)?;
        println!("rear_page: {rear_page}, text: {text}");
        if is_plain_cip_page(&text) {
            return parse_cip_from_text(&text);
        }

        rear_page += 1;
    }

    Err(Error::new(
        ErrorKind::NoCipRecordFound,
        "No cip record found in pdf file",
    ))
}
