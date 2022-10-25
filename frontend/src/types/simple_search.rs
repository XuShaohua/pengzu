// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use serde::{Deserialize, Serialize};

use crate::types::books::GetBooksOrder;
use crate::types::page::default_page_id;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimpleSearchQuery {
    pub query: String,

    #[serde(default = "default_page_id")]
    pub page: i64,
    #[serde(default = "GetBooksOrder::default")]
    pub order: GetBooksOrder,
}

impl Default for SimpleSearchQuery {
    fn default() -> Self {
        Self {
            query: "".to_owned(),
            page: default_page_id(),
            order: GetBooksOrder::default(),
        }
    }
}
