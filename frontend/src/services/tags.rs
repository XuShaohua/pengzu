// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared::books::BookAndAuthorsList;
use shared::books_query::GetBooksQuery;
use shared::recursive_query::RecursiveQuery;
use shared::tags::{Tag, TagAndBookList};

use crate::error::FetchError;
use crate::services::fetch::{request_delete, request_get};

/// Returns tag list.
///
/// # Errors
/// Returns error if server fails.
pub async fn fetch_tags(query: &RecursiveQuery) -> Result<TagAndBookList, FetchError> {
    let query_str = serde_urlencoded::to_string(query)?;
    let url = ["/api/tag", &query_str].join("?");
    request_get(&url).await
}

/// Returns tag info.
///
/// # Errors
/// Returns error if server fails.
pub async fn fetch_tag(tag_id: i32) -> Result<Tag, FetchError> {
    let url = format!("/api/tag/{tag_id}");
    request_get(&url).await
}

/// Get book list of specific tag `tag_id`.
///
/// # Errors
/// Returns error if server fails.
pub async fn fetch_books_by_tag(
    tag_id: i32,
    query: &GetBooksQuery,
) -> Result<BookAndAuthorsList, FetchError> {
    let query_str = serde_urlencoded::to_string(query)?;
    let url = format!("/api/tag/books/{tag_id}?{query_str}");
    request_get(&url).await
}

/// Delete specific tag.
///
/// Note that this api can be trigger by admin only.
///
/// # Errors
/// Returns error if server fails.
pub async fn delete_tag(tag_id: i32) -> Result<(), FetchError> {
    let url = format!("/api/tag/{tag_id}");
    request_delete(&url).await
}
