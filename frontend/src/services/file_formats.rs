// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use crate::error::FetchError;
use crate::services::fetch::request_get;
use crate::types::file_formats::{FileFormat, FileFormatList};

/// Get file format list.
///
/// # Errors
/// Returns error if server fails.
pub async fn fetch_file_formats() -> Result<FileFormatList, FetchError> {
    let url = "/api/format";
    request_get(url).await
}

/// Get file format info.
///
/// # Errors
/// Returns error if server fails.
pub async fn fetch_file_format(format_id: i32) -> Result<FileFormat, FetchError> {
    let url = format!("/api/format/{}", format_id);
    request_get(&url).await
}
