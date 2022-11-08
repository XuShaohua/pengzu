// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use crate::error::FetchError;
use crate::services::fetch::request_get;
use crate::types::authors::{Author, AuthorList};
use crate::types::general_query::GeneralQuery;

/// Get author list
///
/// # Errors
/// Returns error if server failed.
pub async fn fetch_authors(query: &GeneralQuery) -> Result<AuthorList, FetchError> {
    let query_str = serde_urlencoded::to_string(query)?;
    let url = ["/api/author", &query_str].join("?");
    request_get(&url).await
}

/// Get author info
///
/// # Errors
/// Returns error if server failed.
pub async fn fetch_author(author_id: i32) -> Result<Author, FetchError> {
    let url = format!("/api/author/{}", author_id);
    request_get(&url).await
}
