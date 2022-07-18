// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use serde::Deserialize;

use crate::components::models::error::FetchError;
use crate::components::models::fetch::fetch;
use crate::components::models::page::Page;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Series {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct SeriesAndBook {
    pub id: i32,
    pub name: String,
    pub count: i64,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct GetSeriesResp {
    pub page: Page,
    pub list: Vec<SeriesAndBook>,
}

/// Get series list.
///
/// # Error
///
/// Returns error if server fails.
pub async fn fetch_series() -> Result<GetSeriesResp, FetchError> {
    let url = "/api/series";
    let text = fetch(url).await?;
    let obj: GetSeriesResp = serde_json::from_str(&text)?;
    Ok(obj)
}
