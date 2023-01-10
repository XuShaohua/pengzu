// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use chrono::NaiveDateTime;
#[cfg(feature = "diesel")]
use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "diesel", derive(Queryable))]
pub struct Identifier {
    pub id: i32,
    pub book: i32,
    pub scheme: i32,
    pub value: String,
    pub url: Option<String>,
    pub created: NaiveDateTime,
    pub last_modified: NaiveDateTime,
}
