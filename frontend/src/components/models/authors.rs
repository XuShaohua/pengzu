// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use chrono::NaiveDateTime;
use serde::Deserialize;

use crate::components::models::error::FetchError;
use crate::components::models::fetch::fetch;
use crate::components::models::page::Page;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Author {
    pub id: i32,
    pub name: String,
    pub link: String,
    pub created: NaiveDateTime,
    pub last_modified: NaiveDateTime,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct AuthorAndBook {
    pub id: i32,
    pub name: String,
    pub link: String,
    pub count: i64,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct GetAuthorsResp {
    pub page: Page,
    pub list: Vec<AuthorAndBook>,
}

/// Get author list
///
/// # Errors
///
/// Returns error if server failed.
pub async fn fetch_authors() -> Result<GetAuthorsResp, FetchError> {
    // TODO(Shaohua): Add query.
    let url = "/api/author";
    let text = fetch(url).await?;
    let obj: GetAuthorsResp = serde_json::from_str(&text)?;
    Ok(obj)
}
