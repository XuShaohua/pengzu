// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use serde::Deserialize;

pub type PageId = i64;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Page {
    pub page_num: PageId,
    pub each_page: i64,
    pub total: i64,
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

#[must_use]
pub const fn default_page_id() -> PageId {
    1
}
