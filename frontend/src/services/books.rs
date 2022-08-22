// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use crate::error::FetchError;
use crate::services::fetch::fetch;
use crate::types::books::GetBooksResp;

/// Get book list.
///
/// # Errors
///
/// Returns error if server fails.
pub async fn fetch_books() -> Result<GetBooksResp, FetchError> {
    let url = "/api/book";
    let text = fetch(url).await?;
    let obj: GetBooksResp = serde_json::from_str(&text)?;
    Ok(obj)
}

/// Get book list of specific author `author_id`.
///
/// # Errors
///
/// Returns error if server fails.
pub async fn fetch_books_by_author(author_id: i32) -> Result<GetBooksResp, FetchError> {
    let url = format!("/api/author/books/{}", author_id);
    let text = fetch(&url).await?;
    let obj: GetBooksResp = serde_json::from_str(&text)?;
    Ok(obj)
}

/// Get book list of specific publisher `publisher_id`.
///
/// # Errors
///
/// Returns error if server fails.
pub async fn fetch_books_by_publisher(publisher_id: i32) -> Result<GetBooksResp, FetchError> {
    let url = format!("/api/publisher/books/{}", publisher_id);
    let text = fetch(&url).await?;
    let obj: GetBooksResp = serde_json::from_str(&text)?;
    Ok(obj)
}

/// Get book list of specific series `series_id`.
///
/// # Errors
///
/// Returns error if server fails.
pub async fn fetch_books_by_series(series_id: i32) -> Result<GetBooksResp, FetchError> {
    let url = format!("/api/series/books/{}", series_id);
    let text = fetch(&url).await?;
    let obj: GetBooksResp = serde_json::from_str(&text)?;
    Ok(obj)
}

/// Get book list of specific tag `tag_id`.
///
/// # Errors
///
/// Returns error if server fails.
pub async fn fetch_books_by_tag(tag_id: i32) -> Result<GetBooksResp, FetchError> {
    let url = format!("/api/tag/books/{}", tag_id);
    let text = fetch(&url).await?;
    let obj: GetBooksResp = serde_json::from_str(&text)?;
    Ok(obj)
}
