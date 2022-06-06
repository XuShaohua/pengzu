// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use calibre::error::ErrorKind;
use calibre::models::books::{get_next_book, CalibreBook};
use calibre::models::books_authors::get_book_authors;
use calibre::models::books_hash::get_book_hash;
use calibre::models::books_languages::get_book_language;
use calibre::models::books_publishers::get_book_publisher;
use calibre::models::books_ratings::get_book_rating;
use calibre::models::books_tags::get_book_tags;
use calibre::models::comments::get_comment;
use calibre::models::data::get_book_data;
use calibre::models::identifiers::get_identifiers;
use diesel::{PgConnection, SqliteConnection};
use std::fs;

use crate::error::Error;
use crate::import::file_util::{calculate_book_hashes, get_book_file_path};
use crate::models::authors::get_author_by_name;
use crate::models::books::{add_book, Book, NewBook};
use crate::models::books_authors::{add_book_author, NewBookAuthor};
use crate::models::books_languages::{add_book_language, NewBookLanguage};
use crate::models::books_publishers::{add_book_publisher, NewBookPublisher};
use crate::models::books_tags::{add_book_tag, NewBookTag};
use crate::models::comments::{add_comment, NewComment};
use crate::models::file_formats::get_file_format_by_name;
use crate::models::files::{add_file, NewFile};
use crate::models::identifier_types::get_identifier_type_by_name;
use crate::models::identifiers::{add_identifier, NewIdentifier};
use crate::models::languages::get_language_by_name;
use crate::models::publishers::get_publisher_by_name;
use crate::models::ratings::{add_rating, NewRating};
use crate::models::tags::get_tag_by_name;

fn import_authors(
    sqlite_conn: &SqliteConnection,
    pg_conn: &PgConnection,
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
    sqlite_conn: &SqliteConnection,
    pg_conn: &PgConnection,
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
                log::info!("Not comment found for book: {}", calibre_book_id);
            }
            _ => return Err(err.into()),
        },
    }

    Ok(())
}

fn import_identifiers(
    sqlite_conn: &SqliteConnection,
    pg_conn: &PgConnection,
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
    sqlite_conn: &SqliteConnection,
    pg_conn: &PgConnection,
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
            ErrorKind::DbNotFoundError => {
                log::info!("language record not found for book: {}", calibre_book_id);
                Ok(())
            }
            _ => Err(err.into()),
        },
    }
}

fn import_publisher(
    sqlite_conn: &SqliteConnection,
    pg_conn: &PgConnection,
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
            add_book_publisher(pg_conn, &new_publisher)?;
        }
        Err(err) => match err.kind() {
            calibre::error::ErrorKind::DbNotFoundError => {
                log::info!("Not publisher found for book: {}", calibre_book_id);
            }
            _ => return Err(err.into()),
        },
    }

    Ok(())
}

fn import_rating(
    sqlite_conn: &SqliteConnection,
    pg_conn: &PgConnection,
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
            ErrorKind::DbNotFoundError => Ok(()),
            _ => Err(err.into()),
        },
    }
}

fn import_tags(
    sqlite_conn: &SqliteConnection,
    pg_conn: &PgConnection,
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
) -> Result<(), Error> {
    let src_path = get_book_file_path(calibre_library_path, calibre_book_path, file_name, format);
    let dest_path = get_book_file_path(library_path, book_path, file_name, format);
    fs::create_dir_all(&dest_path)?;
    fs::copy(src_path, dest_path).map(drop).map_err(Into::into)
}

fn import_files(
    calibre_library_path: &str,
    library_path: &str,
    sqlite_conn: &SqliteConnection,
    pg_conn: &PgConnection,
    calibre_book_id: i32,
    calibre_book_path: &str,
    book_id: i32,
    book_path: &str,
) -> Result<(), Error> {
    log::info!("import_files({}, {})", calibre_book_id, book_id);
    let calibre_files = get_book_data(sqlite_conn, calibre_book_id)?;
    log::info!("calibre_files len: {}", calibre_files.len());

    let book_hashes = match get_book_hash(sqlite_conn, calibre_book_id) {
        Ok(book_hashes) => book_hashes,
        Err(err) => {
            log::info!(
                "Book hash not found int table: {}, err: {}",
                calibre_book_id,
                err
            );
            calculate_book_hashes(calibre_library_path, calibre_book_path, &calibre_files)?
        }
    };

    for calibre_file in calibre_files {
        let file_format = get_file_format_by_name(pg_conn, &calibre_file.format)?;
        let sha = book_hashes
            .get(&calibre_file.format)
            .map(|item| item.sha.clone())
            .unwrap_or_default();

        copy_book_file(
            calibre_library_path,
            library_path,
            calibre_book_path,
            book_path,
            &calibre_file.name,
            &calibre_file.format,
        )?;

        let new_file = NewFile {
            book: book_id,
            format: file_format.id,
            size: calibre_file.uncompressed_size,
            name: calibre_file.name,
            sha,
        };
        add_file(pg_conn, &new_file)?;
    }

    Ok(())
}

fn import_book(
    calibre_library_path: &str,
    sqlite_conn: &SqliteConnection,
    pg_conn: &PgConnection,
    last_book_id: i32,
) -> Result<Option<(CalibreBook, Book)>, Error> {
    log::info!("import_book({}, {})", calibre_library_path, last_book_id);
    match get_next_book(sqlite_conn, last_book_id) {
        Ok(calibre_book) => {
            let calibre_book_clone: CalibreBook = calibre_book.clone();
            let new_book = NewBook {
                title: calibre_book.title.clone(),
                sort: calibre_book.sort.unwrap_or_else(|| calibre_book.title),
                author_sort: calibre_book.author_sort.unwrap_or_default(),
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
    calibre_library_path: &str,
    library_path: &str,
    sqlite_conn: &SqliteConnection,
    pg_conn: &PgConnection,
) -> Result<(), Error> {
    log::info!("import_books({}, {}", calibre_library_path, library_path);
    let mut last_book_id = 0;

    loop {
        match import_book(calibre_library_path, sqlite_conn, pg_conn, last_book_id) {
            Ok(Some((calibre_book, book))) => {
                let calibre_book_id = calibre_book.id;
                let book_id = book.id;
                last_book_id = calibre_book_id;
                log::info!("last book id updated: {}", last_book_id);

                import_files(
                    calibre_library_path,
                    library_path,
                    sqlite_conn,
                    pg_conn,
                    calibre_book_id,
                    &calibre_book.path,
                    book_id,
                    &book.path,
                )?;
                import_authors(sqlite_conn, pg_conn, calibre_book_id, book_id)?;
                import_comment(sqlite_conn, pg_conn, calibre_book_id, book_id)?;
                import_identifiers(sqlite_conn, pg_conn, calibre_book_id, book_id)?;
                import_language(sqlite_conn, pg_conn, calibre_book_id, book_id)?;
                import_publisher(sqlite_conn, pg_conn, calibre_book_id, book_id)?;
                import_rating(sqlite_conn, pg_conn, calibre_book_id, book_id)?;
                import_tags(sqlite_conn, pg_conn, calibre_book_id, book_id)?;
            }
            Ok(None) => {
                log::info!("DONE");
                return Ok(());
            }
            Err(err) => return Err(err),
        }
    }
}
