// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared::files::{FileQuery, FileWithPath};

#[must_use]
pub fn get_file_format_url(file: &FileWithPath) -> String {
    let query = FileQuery {
        book: file.book,
        file: file.id,
        format: file.format_id,
        path: file.path.clone(),
    };
    let query_str = serde_urlencoded::to_string(query).unwrap_or_default();
    ["/api/file", &query_str].join("?")
}
