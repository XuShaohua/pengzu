// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use crate::components::models::error::FetchError;
use crate::components::models::fetch::fetch;
use serde::Deserialize;

use crate::components::models::page::Page;

#[derive(Debug, PartialEq, Deserialize)]
pub struct TagAndBook {
    pub id: i32,
    pub order_index: i32,
    pub name: String,
    pub parent: i32,
    pub count: i64,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct GetTagsResp {
    pub page: Page,
    pub list: Vec<TagAndBook>,
}

pub async fn fetch_tags() -> Result<GetTagsResp, FetchError> {
    let url = "/api/tag";
    let text = fetch(url).await?;
    let obj: GetTagsResp = serde_json::from_str(&text)?;
    Ok(obj)
}
