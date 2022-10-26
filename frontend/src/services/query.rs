// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use serde::{Deserialize, Serialize};

use crate::types::page::default_page_id;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PageQuery {
    #[serde(default = "default_page_id")]
    pub page: i64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum GetBooksOrder {
    IdDesc,
    IdAsc,
    TitleDesc,
    TitleAsc,
    CreatedDesc,
    CreatedAsc,
    LastModifiedDesc,
    LastModifiedAsc,
    PubdateDesc,
    PubdateAsc,
}

impl Default for GetBooksOrder {
    fn default() -> Self {
        Self::IdDesc
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBooksQuery {
    #[serde(default = "default_page_id")]
    pub page: i64,
    #[serde(default = "GetBooksOrder::default")]
    pub order: GetBooksOrder,
}

#[must_use]
pub fn append_query_to_url(url: &str, query: &GetBooksQuery) -> String {
    [url, &serde_urlencoded::to_string(query).unwrap()].join("?")
}
