// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use chrono::NaiveDateTime;
#[cfg(feature = "diesel")]
use diesel::Queryable;
use serde::{Deserialize, Serialize};

use crate::page::Page;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "diesel", derive(Queryable))]
pub struct Tag {
    pub id: i32,
    pub order_index: i32,
    pub name: String,
    pub parent: i32,
    pub created: NaiveDateTime,
    pub last_modified: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "diesel", derive(Queryable))]
pub struct TagAndBook {
    pub id: i32,
    pub order_index: i32,
    pub name: String,
    pub parent: i32,
    pub count: i64,
    pub children: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TagAndBookList {
    pub page: Page,
    pub list: Vec<TagAndBook>,
}

/// Only used in frontend.
#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct NewTag {
    pub order_index: i32,
    pub name: String,
    pub parent: i32,
}
