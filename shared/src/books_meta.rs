// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#[cfg(feature = "diesel")]
use diesel::Queryable;
use serde::{Deserialize, Serialize};

use crate::authors::Author;
use crate::books::BookWithCover;
use crate::files::FileWithPath;
use crate::languages::Language;
use crate::publishers::Publisher;
use crate::ratings::Rating;
use crate::series::Series;
use crate::tags::Tag;

// TODO(Shaohua): Replace subquery with a meta table in postgres.

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "diesel", derive(Queryable))]
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
