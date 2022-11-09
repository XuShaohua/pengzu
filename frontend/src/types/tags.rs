// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use serde::Deserialize;

use crate::types::page::Page;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Tag {
    pub id: i32,
    pub order_index: i32,
    pub name: String,
    pub parent: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct TagAndBook {
    pub id: i32,
    pub order_index: i32,
    pub name: String,
    pub parent: i32,
    pub count: i64,
    pub children: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct TagList {
    pub page: Page,
    pub list: Vec<TagAndBook>,
}
