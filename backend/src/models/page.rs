// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Page {
    pub page_num: i64,
    pub each_page: i64,
    pub total: i64,
}

#[must_use]
pub const fn default_page_id() -> i64 {
    0
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct PageQuery {
    #[serde(default = "default_page_id")]
    pub page: i64,
}
