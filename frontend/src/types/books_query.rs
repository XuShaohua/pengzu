// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use serde::{Deserialize, Serialize};
use shared::page::{default_page_id, PageId};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetBooksQuery {
    #[serde(default = "default_page_id")]
    pub page: PageId,
    #[serde(default = "GetBooksOrder::default")]
    pub order: GetBooksOrder,
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
