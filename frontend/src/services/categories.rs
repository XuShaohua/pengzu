// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared::books::BookAndAuthorsList;
use shared::books_query::GetBooksQuery;
use shared::categories::{Category, CategoryAndBookList};
use shared::recursive_query::RecursiveQuery;

use crate::error::FetchError;
use crate::services::fetch::request_get;

/// Returns category list.
///
/// # Errors
/// Returns error if server fails.
pub async fn fetch_categories(query: &RecursiveQuery) -> Result<CategoryAndBookList, FetchError> {
    let query_str = serde_urlencoded::to_string(query)?;
    let url = ["/api/category", &query_str].join("?");
    request_get(&url).await
}

/// Returns category info.
///
/// # Errors
/// Returns error if server fails.
pub async fn fetch_category(category_id: i32) -> Result<Category, FetchError> {
    let url = format!("/api/category/{category_id}");
    request_get(&url).await
}

/// Get book list of specific category `category_id`.
///
/// # Errors
/// Returns error if server fails.
pub async fn fetch_books_by_category(
    category_id: i32,
    query: &GetBooksQuery,
) -> Result<BookAndAuthorsList, FetchError> {
    let query_str = serde_urlencoded::to_string(query)?;
    let url = format!("/api/category/{category_id}/book?{query_str}");
    request_get(&url).await
}
