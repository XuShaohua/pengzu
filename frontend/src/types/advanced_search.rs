// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use serde::{Deserialize, Serialize};

use crate::types::books::GetBooksOrder;
use crate::types::page::default_page_id;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdvancedSearchQuery {
    #[serde(default = "default_page_id")]
    pub page: i64,
    #[serde(default = "GetBooksOrder::default")]
    pub order: GetBooksOrder,

    pub title: Option<String>,
    pub author: Option<String>,
    pub publisher: Option<String>,
}

impl Default for AdvancedSearchQuery {
    fn default() -> Self {
        Self {
            page: default_page_id(),
            order: GetBooksOrder::default(),
            title: None,
            author: None,
            publisher: None,
        }
    }
}

impl AdvancedSearchQuery {
    #[must_use]
    pub fn desc(&self) -> String {
        // TODO(Shaohua): Concat query fields.
        if let Some(title) = &self.title {
            return title.to_string();
        }
        if let Some(author) = &self.author {
            return author.to_string();
        }

        "".to_string()
    }
}
