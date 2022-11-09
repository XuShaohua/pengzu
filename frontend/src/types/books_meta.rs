// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use serde::Deserialize;
use shared::authors::Author;
use shared::books::BookWithCover;
use shared::files::FileWithPath;
use shared::languages::Language;
use shared::publishers::Publisher;
use shared::ratings::Rating;
use shared::series::Series;
use shared::tags::Tag;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct BookMetadata {
    pub book: BookWithCover,
    pub authors: Vec<Author>,
    pub tags: Vec<Tag>,
    pub files: Vec<FileWithPath>,
    pub publisher: Option<Publisher>,
    pub series: Option<Series>,
    pub lang: Option<Language>,
    pub rating: Option<Rating>,
}
