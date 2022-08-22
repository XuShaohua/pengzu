// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use crate::services::fetch::fetch;
use crate::types::error::FetchError;
use crate::types::series::GetSeriesResp;

/// Get series list.
///
/// # Error
///
/// Returns error if server fails.
pub async fn fetch_series() -> Result<GetSeriesResp, FetchError> {
    let url = "/api/series";
    let text = fetch(url).await?;
    let obj: GetSeriesResp = serde_json::from_str(&text)?;
    Ok(obj)
}
