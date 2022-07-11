// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::page::Page;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookResp {
    pub id: i32,
    pub title: String,
    pub has_cover: bool,
    pub small_cover: Option<String>,
    pub large_cover: Option<String>,
    pub created: NaiveDateTime,
    pub pubdate: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBooksResp {
    pub page: Page,
    pub list: Vec<BookResp>,
}
