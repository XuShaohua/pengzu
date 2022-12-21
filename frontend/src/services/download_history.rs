// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared::books::BookAndAuthorsList;
use shared::books_query::GetBooksQuery;

use crate::error::FetchError;
use crate::services::fetch::request_get;

/// Get book list of download history of user.
///
/// # Errors
/// Returns error if server fails.
pub async fn fetch_books_by_download_history(
    query: &GetBooksQuery,
) -> Result<BookAndAuthorsList, FetchError> {
    let query_str = serde_urlencoded::to_string(query)?;
    let url = format!("/api/download/books?{query_str}");
    request_get(&url).await
}
