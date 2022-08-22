// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub struct BookMetadata {
    pub book: BookWithCover,
    pub authors: Vec<Author>,
    pub publisher: Option<Publisher>,
    pub series: Option<Series>,
    pub tags: Vec<Tag>,
}
