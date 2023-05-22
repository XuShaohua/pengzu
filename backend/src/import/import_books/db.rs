// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use calibre::models::books_authors::get_book_authors;
use calibre::models::books_languages::get_book_language;
use calibre::models::books_publishers::get_book_publisher;
use calibre::models::books_ratings::get_book_rating;
use calibre::models::books_series::get_book_series;
use calibre::models::books_tags::get_book_tags;
use calibre::models::comments::get_comment;
use calibre::models::identifiers::get_identifiers;
use diesel::{PgConnection, SqliteConnection};

use crate::error::Error;
use crate::models::authors::get_author_by_name;
use crate::models::books_authors::{add_book_author, NewBookAuthor};
use crate::models::books_languages::{add_book_language, NewBookLanguage};
use crate::models::books_publishers::{add_book_publisher, NewBookPublisher};
use crate::models::books_series::{add_book_series, NewBookSeries};
use crate::models::books_tags::{add_book_tag, NewBookTag};
use crate::models::comments::{add_comment, NewComment};
use crate::models::identifier_types::get_identifier_type_by_name;
use crate::models::identifiers::{add_identifier, NewIdentifier};
use crate::models::languages::get_language_by_name;
use crate::models::publishers::get_publisher_by_name;
use crate::models::ratings::{add_rating, NewRating};
use crate::models::series::get_series_by_name;
use crate::models::tags::get_tag_by_name;

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
                language: language.id,
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

/// Import book details to pgsql database.
///
/// Copy book files and metadata (including cover image) if required.
pub fn import_book_detail(
    sqlite_conn: &mut SqliteConnection,
    pg_conn: &mut PgConnection,
    calibre_book_id: i32,
    book_id: i32,
) -> Result<(), Error> {
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
