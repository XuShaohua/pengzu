// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use serde::Deserialize;

use crate::types::authors::Author;
use crate::types::books::BookWithCover;
use crate::types::file_formats::FileFormat;
use crate::types::files::File;
use crate::types::languages::Language;
use crate::types::publishers::Publisher;
use crate::types::ratings::Rating;
use crate::types::series::Series;
use crate::types::tags::Tag;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct BookMetadata {
    pub book: BookWithCover,
    pub authors: Vec<Author>,
    pub tags: Vec<Tag>,
    pub files: Vec<File>,
    pub formats: Vec<FileFormat>,
    pub publisher: Option<Publisher>,
    pub series: Option<Series>,
    pub lang: Option<Language>,
    pub rating: Option<Rating>,
}
