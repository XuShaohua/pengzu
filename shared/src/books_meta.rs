// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#[cfg(feature = "diesel")]
use diesel::Queryable;
use serde::{Deserialize, Serialize};

use crate::authors::Author;
use crate::books::BookWithCover;
use crate::categories::Category;
use crate::files::FileWithPath;
use crate::identifiers::Identifier;
use crate::languages::Language;
use crate::publishers::Publisher;
use crate::ratings::Rating;
use crate::series::Series;
use crate::tags::Tag;
use crate::user_tags::UserTag;

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "diesel", derive(Queryable))]
pub struct BookMetadata {
    pub book: BookWithCover,
    pub authors: Vec<Author>,
    pub tags: Vec<Tag>,
    pub user_tags: Vec<UserTag>,
    pub files: Vec<FileWithPath>,
    pub publisher: Option<Publisher>,
    pub series: Option<Series>,
    pub categories: Vec<Category>,
    pub identifiers: Vec<Identifier>,
    pub lang: Option<Language>,
    pub rating: Option<Rating>,
    pub previous_book: Option<i32>,
    pub next_book: Option<i32>,
}
