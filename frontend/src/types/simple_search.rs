// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use serde::{Deserialize, Serialize};
use shared::books_query::GetBooksOrder;
use shared::page::{default_page_id, PageId};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimpleSearchQuery {
    #[serde(default = "default_page_id")]
    pub page: PageId,
    #[serde(default = "GetBooksOrder::default")]
    pub order: GetBooksOrder,

    pub query: String,
}

impl Default for SimpleSearchQuery {
    fn default() -> Self {
        Self {
            query: String::new(),
            page: default_page_id(),
            order: GetBooksOrder::default(),
        }
    }
}
