// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#![allow(clippy::module_name_repetitions)]

#[must_use]
pub fn html_to_text(html: &str) -> String {
    html2text::from_read(html.as_bytes(), 80)
}
