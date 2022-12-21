// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared::authors::{Author, AuthorAndBookList};
use shared::books::BookAndAuthorsList;
use shared::books_query::GetBooksQuery;
use shared::general_query::GeneralQuery;

use crate::error::FetchError;
use crate::services::fetch::request_get;

/// Get author list
///
/// # Errors
/// Returns error if server failed.
pub async fn fetch_authors(query: &GeneralQuery) -> Result<AuthorAndBookList, FetchError> {
    let query_str = serde_urlencoded::to_string(query)?;
    let url = ["/api/author", &query_str].join("?");
    request_get(&url).await
}

/// Get author info
///
/// # Errors
/// Returns error if server failed.
pub async fn fetch_author(author_id: i32) -> Result<Author, FetchError> {
    let url = format!("/api/author/{author_id}");
    request_get(&url).await
}

/// Get book list of specific author `author_id`.
///
/// # Errors
/// Returns error if server fails.
pub async fn fetch_books_by_author(
    author_id: i32,
    query: &GetBooksQuery,
) -> Result<BookAndAuthorsList, FetchError> {
    let query_str = serde_urlencoded::to_string(query)?;
    let url = format!("/api/author/books/{author_id}?{query_str}");
    request_get(&url).await
}
