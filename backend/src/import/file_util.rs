// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use std::path::{Path, PathBuf};

use crate::error::{Error, ErrorKind};

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

#[cfg(target_family = "windows")]
pub fn chown<P: AsRef<Path>>(
    path: P,
    uid: Option<u32>,
    gid: Option<u32>,
    recursive: bool,
) -> Result<(), Error> {
    Ok(())
}

#[cfg(target_family = "unix")]
#[allow(clippy::similar_names)]
pub fn chown<P: AsRef<Path>>(path: P, uid: Option<u32>, gid: Option<u32>) -> Result<(), Error> {
    let (uid, gid) = match (uid, gid) {
        (Some(uid), Some(gid)) => (uid, gid),
        (None, Some(gid)) => unsafe {
            let euid = nc::getuid();
            (euid, gid)
        },
        (Some(uid), None) => unsafe {
            let egid = nc::getegid();
            (uid, egid)
        },
        (None, None) => {
            // Do nothing.
            return Ok(());
        }
    };

    unsafe { nc::chown(path.as_ref(), uid, gid) }.map_err(|errno| {
        Error::from_string(
            ErrorKind::IoError,
            format!(
                "Failed to change owner of {:?}, reason: {}",
                path.as_ref(),
                nc::strerror(errno)
            ),
        )
    })
}
