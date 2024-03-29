// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#![allow(clippy::module_name_repetitions)]

use crate::error::{Error, ErrorKind};
use crate::parsers::html::html_to_text;

#[derive(Debug, Clone)]
pub struct CipRecord {
    pub title: String,
    pub original_title: String,
    pub authors: Vec<String>,
    pub publisher: String,
    pub pubdate: String,
    pub isbn: String,
    pub category_id: String,
    pub cip_id: String,
    pub price: String,
}

#[must_use]
pub fn is_plain_cip_page(text: &str) -> bool {
    text.contains("图书在版编目") && text.contains("中国版本图书馆")
}

/// Parse CIP record from plain text.
///
/// # Errors
/// Returns error if failed to parse cip record.
pub fn parse_cip_from_text(_text: &str) -> Result<CipRecord, Error> {
    Err(Error::new(ErrorKind::NoCipRecordFound, ""))
}

/// Parse CIP record from html text.
///
/// # Errors
/// Returns error if failed to parse cip record.
pub fn parse_cip_from_html(html: &str) -> Result<CipRecord, Error> {
    let text = html_to_text(html);
    println!("html: {html}, text: {text}");
    parse_cip_from_text(&text)
}
