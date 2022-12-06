// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared::books::{BookAndAuthorsList, BookUpdateReq};
use shared::books_query::GetBooksQuery;

use crate::error::FetchError;
use crate::services::fetch::{request_get, request_put};

/// Get book list.
///
/// # Errors
/// Returns error if server fails.
pub async fn fetch_books(query: &GetBooksQuery) -> Result<BookAndAuthorsList, FetchError> {
    let query_str = serde_urlencoded::to_string(query)?;
    let url = ["/api/book", &query_str].join("?");
    request_get(&url).await
}

/// Update book metadata.
///
/// # Errors
/// Returns error if form is invalid.
pub async fn update_book(query: &BookUpdateReq) -> Result<(), FetchError> {
    let url = format!("/api/book/{}", query.id);
    request_put(&url, query).await
}
