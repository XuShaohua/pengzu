// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use crate::error::FetchError;
use shared::images::ImageQuery;
use std::path::PathBuf;

/// Get cover image url
///
/// # Error
/// Returns error if path is invalid.
pub fn get_cover_url(path: &str) -> Result<String, FetchError> {
    let query = ImageQuery {
        path: PathBuf::from(path),
    };
    let query_str = serde_urlencoded::to_string(query)?;
    let url = ["/api/image", &query_str].join("?");
    Ok(url)
}
