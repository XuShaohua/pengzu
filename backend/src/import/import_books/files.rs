// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use calibre::models::books::CalibreBook;
use calibre::models::data::get_book_data;
use diesel::{PgConnection, SqliteConnection};
use std::fs;

use crate::error::{Error, ErrorKind};
use crate::import::convert::convert_cover;
use crate::import::file_util;
use crate::import::file_util::{get_book_file_path, get_book_metadata_path};
use crate::import::options::{ImportBookFileAction, ImportBookOptions};
use crate::models::books::Book;
use crate::models::file_formats::get_file_format_by_name;
use crate::models::files::{add_file, NewFile};

fn copy_book_file(
    calibre_library_path: &str,
    library_path: &str,
    calibre_book_path: &str,
    book_path: &str,
    file_name: &str,
    format: &str,
    options: &ImportBookOptions,
) -> Result<(), Error> {
    log::info!("copy_book_file({}/{}.{})", book_path, file_name, format);

    let move_files = match options.file_action {
        ImportBookFileAction::Copy => false,
        ImportBookFileAction::Move => true,
        ImportBookFileAction::DoNothing => return Ok(()),
    };

    let src_path = get_book_file_path(calibre_library_path, calibre_book_path, file_name, format);
    let dest_path = get_book_file_path(library_path, book_path, file_name, format);
    let parent_dir = dest_path.parent().ok_or_else(|| {
        Error::from_string(
            ErrorKind::IoError,
            format!("Failed to get parent dir: {:?}", &dest_path),
        )
    })?;
    file_util::create_dir_all_and_chown(parent_dir, options.uid, options.gid)?;
    if move_files {
        fs::rename(&src_path, &dest_path)?;
    } else {
        fs::copy(&src_path, &dest_path).map(drop)?;
    }
    file_util::chown(&dest_path, options.uid, options.gid)
}

fn copy_book_metadata_opf(
    calibre_library_path: &str,
    library_path: &str,
    calibre_book_path: &str,
    book_path: &str,
    options: &ImportBookOptions,
) -> Result<(), Error> {
    let file_name = "metadata.opf";
    log::info!("copy_book_metadata_opf({}/{})", book_path, file_name);
    let move_files = match options.file_action {
        ImportBookFileAction::Copy => false,
        ImportBookFileAction::Move => true,
        ImportBookFileAction::DoNothing => return Ok(()),
    };

    let src_path = get_book_metadata_path(calibre_library_path, calibre_book_path, file_name);
    let dest_path = get_book_metadata_path(library_path, book_path, file_name);
    let parent_dir = dest_path.parent().ok_or_else(|| {
        Error::from_string(
            ErrorKind::IoError,
            format!("Failed to get parent dir: {:?}", &dest_path),
        )
    })?;
    file_util::create_dir_all_and_chown(parent_dir, options.uid, options.gid)?;
    if move_files {
        fs::rename(&src_path, &dest_path)?;
    } else {
        fs::copy(&src_path, &dest_path).map(drop)?;
    }
    file_util::chown(&dest_path, options.uid, options.gid)
}

fn copy_book_cover(
    calibre_library_path: &str,
    library_path: &str,
    calibre_book_path: &str,
    book_path: &str,
    options: &ImportBookOptions,
) -> Result<(), Error> {
    log::info!("copy_book_cover({})", book_path);

    let file_name = "cover.jpg";
    let move_files = match options.file_action {
        ImportBookFileAction::Copy => false,
        ImportBookFileAction::Move => true,
        ImportBookFileAction::DoNothing => return Ok(()),
    };

    let src_path = get_book_metadata_path(calibre_library_path, calibre_book_path, file_name);
    let dest_path = get_book_metadata_path(library_path, book_path, file_name);
    let parent_dir = dest_path.parent().ok_or_else(|| {
        Error::from_string(
            ErrorKind::IoError,
            format!("Failed to get parent dir: {:?}", &dest_path),
        )
    })?;
    file_util::create_dir_all_and_chown(parent_dir, options.uid, options.gid)?;
    if move_files {
        fs::rename(&src_path, &dest_path).map(drop)?;
    } else {
        fs::copy(&src_path, &dest_path).map(drop)?;
    }
    file_util::chown(&dest_path, options.uid, options.gid)?;

    let (webp_path, small_webp_path) = convert_cover(&dest_path)?;
    file_util::chown(webp_path, options.uid, options.gid)?;
    file_util::chown(small_webp_path, options.uid, options.gid)
}

pub fn copy_book_files(
    calibre_library_path: &str,
    library_path: &str,
    sqlite_conn: &mut SqliteConnection,
    pg_conn: &mut PgConnection,

    calibre_book: &CalibreBook,
    book: &Book,
    options: &ImportBookOptions,
) -> Result<(), Error> {
    let calibre_book_id: i32 = calibre_book.id;
    let calibre_book_path: &str = &calibre_book.path;
    let book_id: i32 = book.id;
    let book_path: &str = &book.path;

    log::info!("copy_book_files({}, {})", calibre_book_id, book_id);
    let calibre_files = get_book_data(sqlite_conn, calibre_book_id)?;
    log::info!("calibre_files len: {}", calibre_files.len());

    if let Err(err) = copy_book_cover(
        calibre_library_path,
        library_path,
        calibre_book_path,
        book_path,
        options,
    ) {
        log::warn!("Failed to copy book cover: {:?}", err);
    }

    if let Err(err) = copy_book_metadata_opf(
        calibre_library_path,
        library_path,
        calibre_book_path,
        book_path,
        options,
    ) {
        log::warn!("Failed to copy book metadata.opf: {:?}", err);
    }

    for calibre_file in calibre_files {
        let file_format = get_file_format_by_name(pg_conn, &calibre_file.format)?;

        copy_book_file(
            calibre_library_path,
            library_path,
            calibre_book_path,
            book_path,
            &calibre_file.name,
            &calibre_file.format,
            options,
        )?;

        let new_file = NewFile {
            book: book_id,
            format: file_format.id,
            size: calibre_file.uncompressed_size,
            name: calibre_file.name,
        };
        add_file(pg_conn, &new_file)?;
    }

    Ok(())
}
