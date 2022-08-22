// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use crate::types::page::Page;

use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Publisher {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct PublisherAndBook {
    pub id: i32,
    pub name: String,
    pub count: i64,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct GetPublishersResp {
    pub page: Page,
    pub list: Vec<PublisherAndBook>,
}
