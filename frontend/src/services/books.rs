// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use crate::error::FetchError;
use crate::services::fetch::request_get;
use crate::types::books::BooksList;

/// Get book list.
///
/// # Errors
///
/// Returns error if server fails.
pub async fn fetch_books() -> Result<BooksList, FetchError> {
    let url = "/api/book";
    request_get(url).await
}

/// Get book list of specific author `author_id`.
///
/// # Errors
///
/// Returns error if server fails.
pub async fn fetch_books_by_author(author_id: i32) -> Result<BooksList, FetchError> {
    let url = format!("/api/author/books/{}", author_id);
    request_get(&url).await
}

/// Get book list of specific category `category_id`.
///
/// # Errors
///
/// Returns error if server fails.
pub async fn fetch_books_by_category(category_id: i32) -> Result<BooksList, FetchError> {
    let url = format!("/api/category/books/{}", category_id);
    request_get(&url).await
}

/// Get book list of specific publisher `publisher_id`.
///
/// # Errors
///
/// Returns error if server fails.
pub async fn fetch_books_by_publisher(publisher_id: i32) -> Result<BooksList, FetchError> {
    let url = format!("/api/publisher/books/{}", publisher_id);
    request_get(&url).await
}

/// Get book list of specific series `series_id`.
///
/// # Errors
///
/// Returns error if server fails.
pub async fn fetch_books_by_series(series_id: i32) -> Result<BooksList, FetchError> {
    let url = format!("/api/series/books/{}", series_id);
    request_get(&url).await
}

/// Get book list of specific tag `tag_id`.
///
/// # Errors
///
/// Returns error if server fails.
pub async fn fetch_books_by_tag(tag_id: i32) -> Result<BooksList, FetchError> {
    let url = format!("/api/tag/books/{}", tag_id);
    request_get(&url).await
}
