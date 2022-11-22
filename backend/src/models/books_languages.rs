// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};
use shared::languages::Language;

use crate::error::{Error, ErrorKind};
use crate::schema::books_languages_link;

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = books_languages_link)]
pub struct NewBookLanguage {
    pub book: i32,
    pub language: i32,
}

#[derive(Debug, Serialize, Queryable)]
pub struct BookLanguage {
    pub id: i32,
    pub book: i32,
    pub language: i32,
    pub created: NaiveDateTime,
}

pub fn add_book_language(
    conn: &mut PgConnection,
    new_book_language: &NewBookLanguage,
) -> Result<(), Error> {
    diesel::insert_into(books_languages_link::table)
        .values(new_book_language)
        .execute(conn)?;
    Ok(())
}

pub fn get_book_language(conn: &mut PgConnection, book_id: i32) -> Result<BookLanguage, Error> {
    books_languages_link::table
        .filter(books_languages_link::book.eq(book_id))
        .first::<BookLanguage>(conn)
        .map_err(Into::into)
}

pub fn delete_book_language(conn: &mut PgConnection, book_id: i32) -> Result<(), Error> {
    let _lang = get_book_language(conn, book_id)?;
    diesel::delete(books_languages_link::table.filter(books_languages_link::book.eq(book_id)))
        .execute(conn)?;
    Ok(())
}

pub fn get_language_by_book(
    conn: &mut PgConnection,
    book_id: i32,
) -> Result<Option<Language>, Error> {
    use crate::schema::languages;

    let language: Result<Language, Error> = languages::table
        .filter(
            languages::id.eq_any(
                books_languages_link::table
                    .filter(books_languages_link::book.eq(book_id))
                    .select(books_languages_link::language),
            ),
        )
        .first::<Language>(conn)
        .map_err(Into::into);

    match language {
        Ok(language) => Ok(Some(language)),
        Err(err) => match err.kind() {
            ErrorKind::DbNotFoundError => Ok(None),
            _ => Err(err),
        },
    }
}
