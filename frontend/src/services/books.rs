// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use crate::error::FetchError;
use crate::services::fetch::request_get;
use crate::types::advanced_search::AdvancedSearchQuery;
use crate::types::books::{BooksList, GetBooksQuery};
use crate::types::simple_search::SimpleSearchQuery;

/// Get book list.
///
/// # Errors
/// Returns error if server fails.
pub async fn fetch_books(query: &GetBooksQuery) -> Result<BooksList, FetchError> {
    let query_str = serde_urlencoded::to_string(query).unwrap();
    let url = ["/api/book", &query_str].join("?");
    request_get(&url).await
}

/// Get book list of specific author `author_id`.
///
/// # Errors
/// Returns error if server fails.
pub async fn fetch_books_by_author(
    author_id: i32,
    query: &GetBooksQuery,
) -> Result<BooksList, FetchError> {
    let query_str = serde_urlencoded::to_string(query).unwrap();
    let url = format!("/api/author/books/{}?{}", author_id, query_str);
    request_get(&url).await
}

/// Get book list of specific category `category_id`.
///
/// # Errors
/// Returns error if server fails.
pub async fn fetch_books_by_category(category_id: i32) -> Result<BooksList, FetchError> {
    let url = format!("/api/category/books/{}", category_id);
    request_get(&url).await
}

/// Get book list of specific file format `format_id`.
///
/// # Errors
/// Returns error if server fails.
pub async fn fetch_books_by_file_format(format_id: i32) -> Result<BooksList, FetchError> {
    let url = format!("/api/format/books/{}", format_id);
    request_get(&url).await
}

/// Get book list of specific publisher `publisher_id`.
///
/// # Errors
/// Returns error if server fails.
pub async fn fetch_books_by_publisher(publisher_id: i32) -> Result<BooksList, FetchError> {
    let url = format!("/api/publisher/books/{}", publisher_id);
    request_get(&url).await
}

/// Get book list of specific series `series_id`.
///
/// # Errors
/// Returns error if server fails.
pub async fn fetch_books_by_series(series_id: i32) -> Result<BooksList, FetchError> {
    let url = format!("/api/series/books/{}", series_id);
    request_get(&url).await
}

/// Get book list of specific tag `tag_id`.
///
/// # Errors
/// Returns error if server fails.
pub async fn fetch_books_by_tag(tag_id: i32) -> Result<BooksList, FetchError> {
    let url = format!("/api/tag/books/{}", tag_id);
    request_get(&url).await
}

/// Get book list of specific user tag `tag_id`.
///
/// # Errors
/// Returns error if server fails.
pub async fn fetch_books_by_user_tag(tag_id: i32) -> Result<BooksList, FetchError> {
    let url = format!("/api/user-tag/books/{}", tag_id);
    request_get(&url).await
}

/// Get book list by simple title search.
///
/// # Errors
/// Returns error if server fails.
pub async fn fetch_books_by_simple_search(
    query: &SimpleSearchQuery,
) -> Result<BooksList, FetchError> {
    let s: String = serde_urlencoded::to_string(query)?;
    let url = format!("/api/search/books?{}", s);
    request_get(&url).await
}

/// Get book list by advanced search.
///
/// # Errors
/// Returns error if server fails.
pub async fn fetch_books_by_advanced_search(
    query: &AdvancedSearchQuery,
) -> Result<BooksList, FetchError> {
    let s: String = serde_urlencoded::to_string(query)?;
    let url = format!("/api/advanced-search/books?{}", s);
    request_get(&url).await
}
