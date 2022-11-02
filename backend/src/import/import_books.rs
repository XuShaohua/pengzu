// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use calibre::models::books::{get_next_book, CalibreBook};
use calibre::models::books_authors::get_book_authors;
use calibre::models::books_languages::get_book_language;
use calibre::models::books_publishers::get_book_publisher;
use calibre::models::books_ratings::get_book_rating;
use calibre::models::books_series::get_book_series;
use calibre::models::books_tags::get_book_tags;
use calibre::models::comments::get_comment;
use calibre::models::data::get_book_data;
use calibre::models::identifiers::get_identifiers;
use diesel::{PgConnection, SqliteConnection};
use serde::Serialize;
use std::fs;

use crate::error::{Error, ErrorKind};
use crate::import::convert::convert_cover;
use crate::import::file_util::{get_book_file_path, get_book_metadata_path};
use crate::import::models::books::{add_import_book, NewImportBook};
use crate::import::models::libraries::{update_import_library, ImportLibrary};
use crate::models::authors::get_author_by_name;
use crate::models::books::{add_book, Book, NewBook};
use crate::models::books_authors::{add_book_author, NewBookAuthor};
use crate::models::books_languages::{add_book_language, NewBookLanguage};
use crate::models::books_publishers::{add_book_publisher, NewBookPublisher};
use crate::models::books_series::{add_book_series, NewBookSeries};
use crate::models::books_tags::{add_book_tag, NewBookTag};
use crate::models::comments::{add_comment, NewComment};
use crate::models::file_formats::get_file_format_by_name;
use crate::models::files::{add_file, NewFile};
use crate::models::identifier_types::get_identifier_type_by_name;
use crate::models::identifiers::{add_identifier, NewIdentifier};
use crate::models::languages::get_language_by_name;
use crate::models::publishers::get_publisher_by_name;
use crate::models::ratings::{add_rating, NewRating};
use crate::models::series::get_series_by_name;
use crate::models::tags::get_tag_by_name;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ImportBookFileAction {
    Copy = 1,
    Move = 2,
    DoNothing = 3,
}

#[derive(Debug, Clone, Serialize)]
pub struct ImportBookOptions {
    pub file_action: ImportBookFileAction,
    pub allow_duplication: bool,
}

impl Default for ImportBookOptions {
    fn default() -> Self {
        Self {
            file_action: ImportBookFileAction::Copy,
            allow_duplication: false,
        }
    }
}

fn import_authors(
    sqlite_conn: &mut SqliteConnection,
    pg_conn: &mut PgConnection,
    calibre_book_id: i32,
    book_id: i32,
) -> Result<(), Error> {
    log::info!("import_authors({}, {})", calibre_book_id, book_id);
    let author_list = get_book_authors(sqlite_conn, calibre_book_id)?;
    for calibre_author in &author_list {
        let author = get_author_by_name(pg_conn, &calibre_author.name)?;
        let new_book_author = NewBookAuthor {
            book: book_id,
            author: author.id,
        };
        add_book_author(pg_conn, &new_book_author)?;
    }

    Ok(())
}

fn import_comment(
    sqlite_conn: &mut SqliteConnection,
    pg_conn: &mut PgConnection,
    calibre_book_id: i32,
    book_id: i32,
) -> Result<(), Error> {
    log::info!("import_comment({}, {})", calibre_book_id, book_id);
    match get_comment(sqlite_conn, calibre_book_id) {
        Ok(comment) => {
            let new_comment = NewComment {
                book: book_id,
                text: comment.text,
            };
            add_comment(pg_conn, &new_comment)?;
        }
        Err(err) => match err.kind() {
            calibre::error::ErrorKind::DbNotFoundError => {
                log::info!("No comment found for book: {}", calibre_book_id);
            }
            _ => return Err(err.into()),
        },
    }

    Ok(())
}

fn import_identifiers(
    sqlite_conn: &mut SqliteConnection,
    pg_conn: &mut PgConnection,
    calibre_book_id: i32,
    book_id: i32,
) -> Result<(), Error> {
    log::info!("import_identifier({}, {})", calibre_book_id, book_id);
    let identifier_list = get_identifiers(sqlite_conn, calibre_book_id)?;
    for calibre_identifier in identifier_list {
        let identifier_type = get_identifier_type_by_name(pg_conn, &calibre_identifier.type_)?;
        let new_book_identifier = NewIdentifier {
            book: book_id,
            scheme: identifier_type.id,
            value: calibre_identifier.val,
            url: None,
        };
        add_identifier(pg_conn, &new_book_identifier)?;
    }

    Ok(())
}

fn import_language(
    sqlite_conn: &mut SqliteConnection,
    pg_conn: &mut PgConnection,
    calibre_book_id: i32,
    book_id: i32,
) -> Result<(), Error> {
    log::info!("import_language({}, {})", calibre_book_id, book_id);
    match get_book_language(sqlite_conn, calibre_book_id) {
        Ok(calibre_language) => {
            let language = get_language_by_name(pg_conn, &calibre_language.lang_code)?;
            let new_language = NewBookLanguage {
                book: book_id,
                lang_code: language.id,
            };
            add_book_language(pg_conn, &new_language)
        }
        Err(err) => match err.kind() {
            calibre::error::ErrorKind::DbNotFoundError => {
                log::info!("language record not found for book: {}", calibre_book_id);
                Ok(())
            }
            _ => Err(err.into()),
        },
    }
}

fn import_publisher(
    sqlite_conn: &mut SqliteConnection,
    pg_conn: &mut PgConnection,
    calibre_book_id: i32,
    book_id: i32,
) -> Result<(), Error> {
    log::info!("import_publisher({}, {})", calibre_book_id, book_id);
    match get_book_publisher(sqlite_conn, calibre_book_id) {
        Ok(calibre_publisher) => {
            let publisher = get_publisher_by_name(pg_conn, &calibre_publisher.name)?;
            let new_publisher = NewBookPublisher {
                book: book_id,
                publisher: publisher.id,
            };
            add_book_publisher(pg_conn, &new_publisher)
        }
        Err(err) => match err.kind() {
            calibre::error::ErrorKind::DbNotFoundError => {
                log::info!("No publisher found for book: {}", calibre_book_id);
                Ok(())
            }
            _ => Err(err.into()),
        },
    }
}

fn import_series(
    sqlite_conn: &mut SqliteConnection,
    pg_conn: &mut PgConnection,
    calibre_book_id: i32,
    book_id: i32,
) -> Result<(), Error> {
    log::info!("import_series({}, {})", calibre_book_id, book_id);
    match get_book_series(sqlite_conn, calibre_book_id) {
        Ok(calibre_series) => {
            log::info!("calibre series: {:?}", calibre_series);
            let series = get_series_by_name(pg_conn, &calibre_series.name)?;
            let new_series = NewBookSeries {
                book: book_id,
                series: series.id,
            };
            add_book_series(pg_conn, &new_series)
        }
        Err(err) => match err.kind() {
            calibre::error::ErrorKind::DbNotFoundError => {
                log::info!("No series found for book: {}", calibre_book_id);
                Ok(())
            }
            _ => Err(err.into()),
        },
    }
}

fn import_rating(
    sqlite_conn: &mut SqliteConnection,
    pg_conn: &mut PgConnection,
    calibre_book_id: i32,
    book_id: i32,
) -> Result<(), Error> {
    log::info!("import_rating({}, {})", calibre_book_id, book_id);
    match get_book_rating(sqlite_conn, calibre_book_id) {
        Ok(calibre_rating) => {
            let new_rating = NewRating {
                book: book_id,
                rating: calibre_rating.rating,
            };
            add_rating(pg_conn, &new_rating)
        }
        Err(err) => match err.kind() {
            calibre::error::ErrorKind::DbNotFoundError => Ok(()),
            _ => Err(err.into()),
        },
    }
}

fn import_tags(
    sqlite_conn: &mut SqliteConnection,
    pg_conn: &mut PgConnection,
    calibre_book_id: i32,
    book_id: i32,
) -> Result<(), Error> {
    log::info!("import_tags({}, {})", calibre_book_id, book_id);
    let tag_list = get_book_tags(sqlite_conn, calibre_book_id)?;
    for calibre_tag in &tag_list {
        let tag = get_tag_by_name(pg_conn, &calibre_tag.name)?;
        let new_book_tag = NewBookTag {
            book: book_id,
            tag: tag.id,
        };
        add_book_tag(pg_conn, &new_book_tag)?;
    }

    Ok(())
}

fn copy_book_file(
    calibre_library_path: &str,
    library_path: &str,
    calibre_book_path: &str,
    book_path: &str,
    file_name: &str,
    format: &str,
    file_action: ImportBookFileAction,
) -> Result<(), Error> {
    log::info!("copy_book_file({}/{}.{})", book_path, file_name, format);

    let move_files = match file_action {
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
    fs::create_dir_all(parent_dir)?;
    if move_files {
        fs::rename(src_path, dest_path).map_err(Into::into)
    } else {
        fs::copy(src_path, dest_path).map(drop).map_err(Into::into)
    }
}

fn copy_book_metadata_opf(
    calibre_library_path: &str,
    library_path: &str,
    calibre_book_path: &str,
    book_path: &str,
    file_action: ImportBookFileAction,
) -> Result<(), Error> {
    let file_name = "metadata.opf";
    log::info!("copy_book_metadata_opf({}/{})", book_path, file_name);
    let move_files = match file_action {
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
    fs::create_dir_all(parent_dir)?;
    if move_files {
        fs::rename(src_path, dest_path)
            .map(drop)
            .map_err(Into::into)
    } else {
        fs::copy(src_path, dest_path).map(drop).map_err(Into::into)
    }
}

fn copy_book_cover(
    calibre_library_path: &str,
    library_path: &str,
    calibre_book_path: &str,
    book_path: &str,
    file_action: ImportBookFileAction,
) -> Result<(), Error> {
    log::info!("copy_book_cover({})", book_path);

    let file_name = "cover.jpg";
    let move_files = match file_action {
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
    fs::create_dir_all(parent_dir)?;
    if move_files {
        fs::rename(&src_path, &dest_path).map(drop)?;
    } else {
        fs::copy(&src_path, &dest_path).map(drop)?;
    }

    convert_cover(&dest_path)
}

#[allow(clippy::too_many_arguments)]
fn copy_book_files(
    calibre_library_path: &str,
    library_path: &str,
    sqlite_conn: &mut SqliteConnection,
    pg_conn: &mut PgConnection,
    calibre_book_id: i32,
    calibre_book_path: &str,
    book_id: i32,
    book_path: &str,
    file_action: ImportBookFileAction,
) -> Result<(), Error> {
    log::info!("copy_book_files({}, {})", calibre_book_id, book_id);
    let calibre_files = get_book_data(sqlite_conn, calibre_book_id)?;
    log::info!("calibre_files len: {}", calibre_files.len());

    if let Err(err) = copy_book_cover(
        calibre_library_path,
        library_path,
        calibre_book_path,
        book_path,
        file_action,
    ) {
        log::warn!("Failed to copy book cover: {:?}", err);
    }

    if let Err(err) = copy_book_metadata_opf(
        calibre_library_path,
        library_path,
        calibre_book_path,
        book_path,
        file_action,
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
            file_action,
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

/// Import book details to pgsql database.
///
/// Copy book files and metadata (including cover image) if required.
fn import_book_detail(
    calibre_library_path: &str,
    library_path: &str,
    sqlite_conn: &mut SqliteConnection,
    pg_conn: &mut PgConnection,
    calibre_book: &CalibreBook,
    book: &Book,
    option: &ImportBookOptions,
) -> Result<(), Error> {
    let calibre_book_id = calibre_book.id;
    let book_id = book.id;

    copy_book_files(
        calibre_library_path,
        library_path,
        sqlite_conn,
        pg_conn,
        calibre_book_id,
        &calibre_book.path,
        book_id,
        &book.path,
        option.file_action,
    )?;
    import_authors(sqlite_conn, pg_conn, calibre_book_id, book_id)?;
    import_comment(sqlite_conn, pg_conn, calibre_book_id, book_id)?;
    import_identifiers(sqlite_conn, pg_conn, calibre_book_id, book_id)?;
    import_language(sqlite_conn, pg_conn, calibre_book_id, book_id)?;
    import_publisher(sqlite_conn, pg_conn, calibre_book_id, book_id)?;
    import_series(sqlite_conn, pg_conn, calibre_book_id, book_id)?;
    import_rating(sqlite_conn, pg_conn, calibre_book_id, book_id)?;
    import_tags(sqlite_conn, pg_conn, calibre_book_id, book_id)?;
    Ok(())
}

/// Add book record to pgsql database.
fn import_book(
    calibre_library_path: &str,
    sqlite_conn: &mut SqliteConnection,
    pg_conn: &mut PgConnection,
    last_book_id: i32,
) -> Result<Option<(CalibreBook, Book)>, Error> {
    log::info!("import_book({}, {})", calibre_library_path, last_book_id);
    match get_next_book(sqlite_conn, last_book_id) {
        Ok(calibre_book) => {
            let calibre_book_clone: CalibreBook = calibre_book.clone();
            let new_book = NewBook {
                title: calibre_book.title.clone(),
                path: calibre_book.path,
                uuid: calibre_book.uuid,
                has_cover: calibre_book.has_cover,
            };
            let book = add_book(pg_conn, &new_book)?;
            Ok(Some((calibre_book_clone, book)))
        }
        Err(err) => match err.kind() {
            calibre::error::ErrorKind::DbNotFoundError => {
                log::info!(
                    "No more books in calibre library: {}, last book id is: {}",
                    calibre_library_path,
                    last_book_id
                );
                Ok(None)
            }
            _ => Err(err.into()),
        },
    }
}

pub fn import_books(
    sqlite_conn: &mut SqliteConnection,
    pg_conn: &mut PgConnection,
    import_library: &ImportLibrary,
    option: &ImportBookOptions,
    mut last_book_id: i32,
) -> Result<(), Error> {
    log::info!("import_books({:?})", &import_library);

    loop {
        match import_book(
            &import_library.calibre_library_path,
            sqlite_conn,
            pg_conn,
            last_book_id,
        ) {
            Ok(Some((calibre_book, book))) => {
                last_book_id = calibre_book.id;
                log::info!("last book id updated: {}", last_book_id);

                let ok = if let Err(err) = import_book_detail(
                    &import_library.calibre_library_path,
                    &import_library.library_path,
                    sqlite_conn,
                    pg_conn,
                    &calibre_book,
                    &book,
                    option,
                ) {
                    log::warn!("Failed to import book: {:?}, err: {:?}", &calibre_book, err);
                    false
                } else {
                    true
                };

                let new_book = NewImportBook {
                    library: import_library.id,
                    calibre_book: calibre_book.id,
                    ok,
                    book: Some(book.id),
                };
                if let Err(err) = add_import_book(pg_conn, &new_book) {
                    log::error!("Failed to add import-book {:?}, err: {}", new_book, err);
                }
            }
            Ok(None) => {
                log::info!("All books are imported: {:?}", import_library);
                return update_import_library(pg_conn, import_library.id, true);
            }
            Err(err) => return Err(err),
        }
    }
}
