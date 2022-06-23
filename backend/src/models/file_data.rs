// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

pub fn get_large_cover(path: &str, has_cover: bool) -> Option<String> {
    if has_cover {
        let cover_path = format!("{}/small_cover.jpg", path);
        Some(cover_path)
    } else {
        None
    }
}

pub fn get_small_cover(path: &str, has_cover: bool) -> Option<String> {
    if has_cover {
        let cover_path = format!("{}/cover.jpg", path);
        Some(cover_path)
    } else {
        None
    }
}
