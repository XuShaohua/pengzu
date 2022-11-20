// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use web_sys::{ScrollBehavior, ScrollToOptions, Window};

use crate::services::images::get_cover_url;

pub const KILO_BYTES: i32 = 1 << 10;
pub const MEGA_BYTES: i32 = 1 << 20;

pub const DEFAULT_COVER_IMG: &str = "/assets/images/book_cover.webp";

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

pub fn set_document_title(title: &str) {
    let prefix = "PengzuLibrary";
    if title.is_empty() {
        gloo_utils::document().set_title(prefix);
    } else {
        let title = [prefix, title].join(" | ");
        gloo_utils::document().set_title(&title);
    }
}

pub fn scroll_to_top() {
    let window: Window = gloo_utils::window();
    let mut options = ScrollToOptions::new();
    options.top(0.0).behavior(ScrollBehavior::Instant);
    window.scroll_to_with_scroll_to_options(&options);
}
