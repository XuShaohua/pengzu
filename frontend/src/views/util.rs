// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#[must_use]
pub fn get_cover_image_url(cover: &Option<String>) -> String {
    cover.as_ref().map_or_else(
        || "#".to_string(),
        |cover| format!("/api/file?path={}", cover),
    )
}

#[must_use]
pub fn get_file_format_url(path: &str, name: &str, format: &str) -> String {
    format!(
        "/api/file?path={}/{}.{}",
        path,
        name,
        format.to_ascii_lowercase()
    )
}
