// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

pub const KILO_BYTES: i32 = 1 << 10;
pub const MEGA_BYTES: i32 = 1 << 20;

#[must_use]
pub fn get_cover_image_url(cover: &Option<String>) -> String {
    cover.as_ref().map_or_else(
        || "#".to_string(),
        |cover| format!("/api/file?path={}", cover),
    )
}

#[must_use]
pub fn get_file_format_url(path: &str) -> String {
    format!("/api/file?path={}", path,)
}

#[must_use]
#[allow(clippy::cast_precision_loss)]
pub fn to_readable_size(size: i32) -> String {
    if size > MEGA_BYTES {
        let mb_size: f32 = size as f32 / MEGA_BYTES as f32;
        format!("{:.1} MB", mb_size)
    } else if size > KILO_BYTES {
        let kb_size: f32 = size as f32 / KILO_BYTES as f32;
        format!("{:.1} kB", kb_size)
    } else {
        format!("{} B", size)
    }
}
