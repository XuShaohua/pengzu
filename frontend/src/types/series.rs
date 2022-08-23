// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use serde::Deserialize;

use crate::types::page::Page;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Series {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct SeriesAndBook {
    pub id: i32,
    pub name: String,
    pub count: i64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct SeriesList {
    pub page: Page,
    pub list: Vec<SeriesAndBook>,
}
