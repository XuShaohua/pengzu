// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared::recursive_query::RecursiveQuery;

use crate::error::FetchError;
use crate::services::fetch::request_get;
use crate::types::tags::{Tag, TagList};

/// Returns tag list.
///
/// # Errors
/// Returns error if server fails.
pub async fn fetch_tags(query: &RecursiveQuery) -> Result<TagList, FetchError> {
    let query_str = serde_urlencoded::to_string(query)?;
    let url = ["/api/tag", &query_str].join("?");
    request_get(&url).await
}

/// Returns tag info.
///
/// # Errors
/// Returns error if server fails.
pub async fn fetch_tag(tag_id: i32) -> Result<Tag, FetchError> {
    let url = format!("/api/tag/{}", tag_id);
    request_get(&url).await
}
