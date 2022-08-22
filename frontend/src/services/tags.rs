// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use crate::error::FetchError;
use crate::services::fetch::fetch;
use crate::types::tags::GetTagsResp;

/// Returns tag list.
///
/// # Errors
///
/// Returns error if server fails.
pub async fn fetch_tags() -> Result<GetTagsResp, FetchError> {
    let url = "/api/tag";
    let text = fetch(url).await?;
    let obj: GetTagsResp = serde_json::from_str(&text)?;
    Ok(obj)
}
