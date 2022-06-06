// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use calibre::models::books_hash::{FindDuplicateItem, FindDuplicateMap};
use calibre::models::data::Data;
use shell_rs::hashsum::{sha1sum, Options};
use std::collections::HashMap;
use std::fs;
use std::os::unix::fs::MetadataExt;

use crate::error::Error;

pub fn get_book_file_path(library_path: &str, book_path: &str, format: &Data) -> String {
    vec![
        library_path,
        book_path,
        &format!("{}.{}", format.name, format.format.to_ascii_lowercase()),
    ]
    .join("/")
}

pub fn calculate_book_hashes(
    library_path: &str,
    book_path: &str,
    formats: &[Data],
) -> Result<FindDuplicateMap, Error> {
    let mut map = HashMap::new();
    for format in formats {
        let file_path = get_book_file_path(library_path, book_path, format);
        log::info!("file path: {:?}", file_path);
        let metadata = fs::metadata(&file_path)?;
        let file_size = metadata.blksize() * metadata.blocks();
        let file_size = file_size as i32;
        let file_hash = sha1sum(&file_path, &Options::default())?;

        map.insert(
            format.format.clone(),
            FindDuplicateItem {
                sha: file_hash,
                size: file_size,
            },
        );
    }

    Ok(map)
}
