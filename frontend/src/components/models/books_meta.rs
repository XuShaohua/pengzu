// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use serde::Deserialize;

use crate::components::models::authors::Author;
use crate::components::models::books::BookWithCover;
use crate::components::models::error::FetchError;
use crate::components::models::fetch::fetch;
use crate::components::models::publishers::Publisher;
use crate::components::models::series::Series;
use crate::components::models::tags::Tag;

#[derive(Debug, PartialEq, Deserialize)]
pub struct BookMetadata {
    pub book: BookWithCover,
    pub authors: Vec<Author>,
    pub publisher: Option<Publisher>,
    pub series: Option<Series>,
    pub tags: Vec<Tag>,
}

/// Get book metadata.
///
/// # Errors
///
/// Returns error if server failed.
pub async fn fetch_book_metadata(book_id: i32) -> Result<BookMetadata, FetchError> {
    let url = format!("/api/book/{}", book_id);
    let text = fetch(&url).await?;
    let metadata: BookMetadata = serde_json::from_str(&text)?;
    Ok(metadata)
}
