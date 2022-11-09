// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use chrono::NaiveDateTime;
#[cfg(feature = "use_query")]
use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "use_query", derive(Queryable))]
pub struct Language {
    pub id: i32,
    pub lang_code: String,
    pub created: NaiveDateTime,
}
