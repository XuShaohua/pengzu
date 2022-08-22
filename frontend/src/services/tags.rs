// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use crate::error::FetchError;
use crate::services::fetch::request_get;
use crate::types::tags::TagList;

/// Returns tag list.
///
/// # Errors
///
/// Returns error if server fails.
pub async fn fetch_tags() -> Result<TagList, FetchError> {
    let url = "/api/tag";
    request_get(url).await
}
