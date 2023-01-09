// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared::books::BookAndAuthorsList;
use shared::books_query::GetBooksQuery;
use shared::recursive_query::RecursiveQuery;
use shared::user_tags::{NewUserTag, UserTag, UserTagAndBookList};

use crate::error::FetchError;
use crate::services::fetch::{request_delete, request_get, request_post, request_put};

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
    let url = format!("/api/user-tag/{tag_id}");
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
    let url = format!("/api/user-tag/books/{tag_id}?{query_str}");
    request_get(&url).await
}

/// Add a new user tag record.
///
/// # Errors
/// Returns error if server error or `new_tag` is invalid.
pub async fn add_tag(new_tag: &NewUserTag) -> Result<UserTag, FetchError> {
    let url = "/api/user-tag";
    request_post(url, new_tag).await
}

/// Update fields of existing tag.
///
/// # Errors
/// Returns error if:
/// - Server error
/// - Invalid tag id
/// - Invalid fields in `new_tag`
pub async fn update_tag(tag_id: i32, new_tag: &NewUserTag) -> Result<(), FetchError> {
    let url = format!("/api/user-tag/{tag_id}");
    request_put(&url, new_tag).await
}

/// Delete specific user tag.
///
/// Note that this api can be trigger by admin only.
///
/// # Errors
/// Returns error if server fails.
pub async fn delete_tag(tag_id: i32) -> Result<(), FetchError> {
    let url = format!("/api/user-tag/{tag_id}");
    request_delete(&url).await
}
