// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use chrono::NaiveDateTime;
use serde::Deserialize;

use super::error::FetchError;
use super::page::Page;
use crate::components::models::fetch::fetch;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct AuthorAndBookId {
    pub id: i32,
    pub name: String,
    pub book: i32,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct BookResp {
    pub id: i32,
    pub title: String,
    pub has_cover: bool,
    pub small_cover: Option<String>,
    pub large_cover: Option<String>,
    pub created: NaiveDateTime,
    pub pubdate: NaiveDateTime,

    pub authors: Vec<AuthorAndBookId>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct GetBooksResp {
    pub page: Page,
    pub list: Vec<BookResp>,
}

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

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub has_cover: bool,
    pub small_cover: Option<String>,
    pub large_cover: Option<String>,
    pub created: NaiveDateTime,
    pub pubdate: NaiveDateTime,
}
