// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use serde::{Deserialize, Serialize};

use crate::page::{default_page_id, PageId};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetBooksQuery {
    /// Human readable page id, used in frontend.
    #[serde(default = "default_page_id")]
    pub page: PageId,
    #[serde(default = "GetBooksOrder::default")]
    pub order: GetBooksOrder,
}

impl GetBooksQuery {
    pub fn backend_page_id(&self) -> PageId {
        if self.page < 1 {
            0
        } else {
            self.page - 1
        }
    }
    pub fn frontend_page_id(&self) -> PageId {
        if self.page < 1 {
            1
        } else {
            self.page
        }
    }
}

impl Default for GetBooksQuery {
    fn default() -> Self {
        Self {
            page: default_page_id(),
            order: GetBooksOrder::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum GetBooksOrder {
    IdDesc,
    IdAsc,
    TitleDesc,
    TitleAsc,
    AuthorDesc,
    AuthorAsc,
    PubdateDesc,
    PubdateAsc,
}

impl Default for GetBooksOrder {
    fn default() -> Self {
        Self::IdDesc
    }
}
