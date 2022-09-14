// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use diesel::PgConnection;
use serde::Serialize;

use crate::error::{Error, ErrorKind};
use crate::models::authors::Author;
use crate::models::books::{get_book_by_id, BookWithCover};
use crate::models::books_authors::get_authors_by_book;
use crate::models::books_languages::get_language_by_book;
use crate::models::books_publishers::get_publisher_by_book;
use crate::models::books_series::get_series_by_book;
use crate::models::books_tags::get_tags_by_book;
use crate::models::files::{get_book_files_and_formats, FileWithPath};
use crate::models::languages::Language;
use crate::models::publishers::Publisher;
use crate::models::ratings::{get_rating, Rating};
use crate::models::series::Series;
use crate::models::tags::Tag;

// TODO(Shaohua): Replace subquery with a meta table in postgres.
#[derive(Debug, Serialize, Queryable)]
pub struct BookMetadata {
    pub book: BookWithCover,
    pub authors: Vec<Author>,
    pub tags: Vec<Tag>,
    pub files: Vec<FileWithPath>,
    pub publisher: Option<Publisher>,
    pub series: Option<Series>,
    pub lang: Option<Language>,
    pub rating: Option<Rating>,
}

pub fn get_book_metadata(conn: &mut PgConnection, book_id: i32) -> Result<BookMetadata, Error> {
    let book = get_book_by_id(conn, book_id)?;
    let authors = get_authors_by_book(conn, book_id)?;
    let tags = get_tags_by_book(conn, book_id)?;
    let files = get_book_files_and_formats(conn, book_id)?;
    let publisher = get_publisher_by_book(conn, book_id)?;
    let series = get_series_by_book(conn, book_id)?;
    let rating = match get_rating(conn, book_id) {
        Ok(rating) => Some(rating),
        Err(err) => match err.kind() {
            ErrorKind::DbNotFoundError => None,
            _ => return Err(err),
        },
    };
    let lang = get_language_by_book(conn, book_id)?;

    Ok(BookMetadata {
        book,
        authors,
        tags,
        files,
        publisher,
        series,
        lang,
        rating,
    })
}
