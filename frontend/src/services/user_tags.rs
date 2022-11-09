// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared::books::BookAndAuthorsList;
use shared::books_query::GetBooksQuery;
use shared::recursive_query::RecursiveQuery;
use shared::user_tags::{UserTag, UserTagAndBookList};

use crate::error::FetchError;
use crate::services::fetch::request_get;

/// Returns user tag list.
///
/// # Errors
/// Returns error if server fails.
pub async fn fetch_user_tags(query: &RecursiveQuery) -> Result<UserTagAndBookList, FetchError> {
    let query_str = serde_urlencoded::to_string(query)?;
    let url = ["/api/user-tag", &query_str].join("?");
    request_get(&url).await
}

/// Returns user tag info.
///
/// # Errors
/// Returns error if server fails.
pub async fn fetch_user_tag(tag_id: i32) -> Result<UserTag, FetchError> {
    let url = format!("/api/user-tag/{}", tag_id);
    request_get(&url).await
}

/// Get book list of specific user tag `tag_id`.
///
/// # Errors
/// Returns error if server fails.
pub async fn fetch_books_by_user_tag(
    tag_id: i32,
    query: &GetBooksQuery,
) -> Result<BookAndAuthorsList, FetchError> {
    let query_str = serde_urlencoded::to_string(query)?;
    let url = format!("/api/user-tag/books/{}?{}", tag_id, query_str);
    request_get(&url).await
}
