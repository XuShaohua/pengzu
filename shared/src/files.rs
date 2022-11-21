// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use chrono::NaiveDateTime;
#[cfg(feature = "diesel")]
use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "diesel", derive(Queryable))]
pub struct File {
    pub id: i32,
    pub book: i32,
    pub format: i32,
    pub size: i32,
    pub name: String,
    pub created: NaiveDateTime,
    pub last_modified: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "diesel", derive(Queryable))]
pub struct FileWithPath {
    pub id: i32,
    pub book: i32,
    pub size: i32,
    pub format_id: i32,
    pub format_name: String,
    pub name: String,
    pub path: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FileQuery {
    pub book: i32,
    pub file: i32,
    pub format: i32,
    pub path: String,
}
