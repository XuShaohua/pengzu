// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared::images::ImageQuery;
use std::path::PathBuf;

use crate::error::FetchError;

pub const DEFAULT_COVER_IMG: &str = "/assets/images/book_cover.webp";

/// Get cover image url
///
/// # Error
/// Returns error if path is invalid.
fn get_cover_url(path: &str) -> Result<String, FetchError> {
    let query = ImageQuery {
        path: PathBuf::from(path),
    };
    let query_str = serde_urlencoded::to_string(query)?;
    let url = ["/api/image", &query_str].join("?");
    Ok(url)
}

#[must_use]
pub fn get_cover_image_url(cover: &Option<String>) -> String {
    if let Some(cover) = &cover {
        if !cover.is_empty() {
            if let Ok(url) = get_cover_url(cover) {
                return url;
            }
        }
    }

    DEFAULT_COVER_IMG.to_string()
}
