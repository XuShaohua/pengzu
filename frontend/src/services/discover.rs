// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared::books::BookAndAuthorsList;

use crate::error::FetchError;
use crate::services::fetch::request_get;

/// Get random book list.
///
/// # Errors
/// Returns error if server fails.
pub async fn fetch_books_by_discover() -> Result<BookAndAuthorsList, FetchError> {
    request_get("/api/discover/books").await
}
