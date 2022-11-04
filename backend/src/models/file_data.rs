// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use urlencoding::encode;

pub fn get_book_file(path: &str) -> String {
    encode(path).to_string()
}

pub fn get_large_cover(path: &str, has_cover: bool) -> Option<String> {
    if has_cover {
        let cover_path = format!("{}/cover.webp", path);
        Some(encode(&cover_path).to_string())
    } else {
        None
    }
}

pub fn get_small_cover(path: &str, has_cover: bool) -> Option<String> {
    if has_cover {
        let cover_path = format!("{}/small_cover.webp", path);
        Some(encode(&cover_path).to_string())
    } else {
        None
    }
}
