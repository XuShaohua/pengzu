// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use serde::{Deserialize, Serialize};

pub const AUTHORS_EACH_PAGE: i64 = 100;
pub const BOOKS_EACH_PAGE: i64 = 50;
pub const PUBLISHERS_EACH_PAGE: i64 = 100;
pub const SERIES_EACH_PAGE: i64 = 100;
pub const TAGS_EACH_PAGE: i64 = 100;

pub type PageId = i64;

/// Used in pagination.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Page {
    /// Current page number of supported query.
    pub page_num: PageId,

    /// Item count in each page.
    pub each_page: i64,

    /// Total items of supported query.
    pub total: i64,
}

/// Default page number is 1.
///
/// As page numbers starts with 1 in frontend.
#[must_use]
pub const fn default_page_id() -> PageId {
    1
}

impl Page {
    #[must_use]
    pub const fn total_pages(&self) -> PageId {
        let page = self.total / self.each_page;
        if page * self.each_page < self.total {
            page + 1
        } else {
            page
        }
    }
}

/// Query parameter which only contains page number.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PageQuery {
    #[serde(default = "default_page_id")]
    pub page: PageId,
}
