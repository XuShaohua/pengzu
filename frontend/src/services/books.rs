// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use crate::error::FetchError;
use crate::services::fetch::{request_get, request_post};
use crate::services::query::{append_query_to_url, GetBooksQuery};
use crate::types::advanced_search::AdvancedSearchQuery;
use crate::types::books::BooksList;

/// Get book list.
///
/// # Errors
/// Returns error if server fails.
pub async fn fetch_books(query: Option<GetBooksQuery>) -> Result<BooksList, FetchError> {
    let url = append_query_to_url("/api/book", query);
    log::info!("book url: {}", url);
    request_get(&url).await
}

/// Get book list of specific author `author_id`.
///
/// # Errors
/// Returns error if server fails.
pub async fn fetch_books_by_author(author_id: i32) -> Result<BooksList, FetchError> {
    let url = format!("/api/author/books/{}", author_id);
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
pub async fn fetch_books_by_simple_search(query: &str) -> Result<BooksList, FetchError> {
    let url = format!("/api/search/books/{}", query);
    request_get(&url).await
}

/// Get book list by advanced search.
///
/// # Errors
/// Returns error if server fails.
pub async fn fetch_books_by_advanced_search(
    query: &AdvancedSearchQuery,
    page_id: i32,
) -> Result<BooksList, FetchError> {
    let url = format!("/api/advanced-search/books/{}", page_id);
    request_post(&url, query).await
}
