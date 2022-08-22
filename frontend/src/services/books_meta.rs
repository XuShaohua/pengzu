// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use crate::services::fetch::fetch;
use crate::types::books_meta::BookMetadata;
use crate::types::error::FetchError;

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
