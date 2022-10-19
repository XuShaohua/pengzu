// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdvancedSearchQuery {
    pub title: Option<String>,
    pub author: Option<String>,
    pub publisher: Option<String>,
}

impl FromStr for AdvancedSearchQuery {
    type Err = String;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        todo!()
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
