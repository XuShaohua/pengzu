// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use std::path::PathBuf;

pub fn get_book_file_path(
    library_path: &str,
    book_path: &str,
    file_name: &str,
    format: &str,
) -> PathBuf {
    let p: String = [
        library_path,
        book_path,
        &format!("{file_name}.{}", format.to_ascii_lowercase()),
    ]
    .join("/");
    PathBuf::from(p)
}

pub fn get_book_metadata_path(library_path: &str, book_path: &str, file_name: &str) -> PathBuf {
    let p: String = [library_path, book_path, file_name].join("/");
    PathBuf::from(p)
}
