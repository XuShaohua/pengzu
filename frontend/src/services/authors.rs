// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use crate::error::FetchError;
use crate::services::fetch::request_get;
use crate::types::authors::AuthorList;

/// Get author list
///
/// # Errors
/// Returns error if server failed.
pub async fn fetch_authors() -> Result<AuthorList, FetchError> {
    // TODO(Shaohua): Add query.
    let url = "/api/author";
    request_get(url).await
}
