// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use chrono::NaiveDateTime;
#[cfg(feature = "use_query")]
use diesel::Queryable;
use serde::{Deserialize, Serialize};

use crate::page::Page;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "use_query", derive(Queryable))]
pub struct Author {
    pub id: i32,
    pub name: String,
    pub link: String,
    pub created: NaiveDateTime,
    pub last_modified: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "use_query", derive(Queryable))]
pub struct AuthorAndBook {
    pub id: i32,
    pub name: String,
    pub link: String,
    pub count: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthorAndBookList {
    pub page: Page,
    pub list: Vec<AuthorAndBook>,
}
