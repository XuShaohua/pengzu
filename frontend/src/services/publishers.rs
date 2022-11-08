// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use crate::error::FetchError;
use crate::services::fetch::request_get;
use crate::types::general_query::GeneralQuery;
use crate::types::publishers::{Publisher, PublisherList};

/// Get publisher list.
///
/// # Errors
/// Returns error if server fails.
pub async fn fetch_publishers(query: &GeneralQuery) -> Result<PublisherList, FetchError> {
    let query_str = serde_urlencoded::to_string(query)?;
    let url = ["/api/publisher", &query_str].join("?");
    request_get(&url).await
}

/// Get publisher info.
///
/// # Errors
/// Returns error if server fails.
pub async fn fetch_publisher(publisher_id: i32) -> Result<Publisher, FetchError> {
    let url = format!("/api/publisher/{}", publisher_id);
    request_get(&url).await
}
