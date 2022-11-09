// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared::advanced_search::AdvancedSearchQuery;
use shared::books::BookAndAuthorsList;

use crate::error::FetchError;
use crate::services::fetch::request_get;

/// Get book list by advanced search.
///
/// # Errors
/// Returns error if server fails.
pub async fn fetch_books_by_advanced_search(
    query: &AdvancedSearchQuery,
) -> Result<BookAndAuthorsList, FetchError> {
    let s: String = serde_urlencoded::to_string(query)?;
    let url = format!("/api/advanced-search/books?{}", s);
    request_get(&url).await
}
