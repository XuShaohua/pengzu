// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use chrono::NaiveDateTime;
use diesel::{
    ExpressionMethods, Insertable, JoinOnDsl, PgConnection, QueryDsl, Queryable, RunQueryDsl,
};
use serde::{Deserialize, Serialize};

use crate::error::{Error, ErrorKind};
use crate::models::languages::Language;
use crate::schema::books_languages_link;

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = books_languages_link)]
pub struct NewBookLanguage {
    pub book: i32,
    pub lang_code: i32,
}

#[derive(Debug, Serialize, Queryable)]
pub struct BookLanguage {
    pub id: i32,
    pub book: i32,
    pub lang_code: i32,
    pub created: NaiveDateTime,
}

pub fn add_book_language(
    conn: &mut PgConnection,
    new_book_language: &NewBookLanguage,
) -> Result<(), Error> {
    use crate::schema::books_languages_link::dsl::books_languages_link;
    diesel::insert_into(books_languages_link)
        .values(new_book_language)
        .execute(conn)?;
    Ok(())
}

pub fn get_book_language(conn: &mut PgConnection, book_id: i32) -> Result<BookLanguage, Error> {
    use crate::schema::books_languages_link::dsl::{book, books_languages_link};
    books_languages_link
        .filter(book.eq(book_id))
        .first::<BookLanguage>(conn)
        .map_err(Into::into)
}

pub fn delete_book_language(conn: &mut PgConnection, book_id: i32) -> Result<(), Error> {
    use crate::schema::books_languages_link::dsl::{book, books_languages_link};
    let _lang = get_book_language(conn, book_id)?;
    diesel::delete(books_languages_link.filter(book.eq(book_id))).execute(conn)?;
    Ok(())
}

pub fn get_language_by_book(
    conn: &mut PgConnection,
    book_id: i32,
) -> Result<Option<Language>, Error> {
    use crate::schema::languages;

    let language: Result<Language, Error> = languages::table
        .inner_join(
            books_languages_link::table.on(books_languages_link::lang_code.eq(languages::id)),
        )
        .filter(books_languages_link::book.eq(book_id))
        .select((languages::id, languages::lang_code, languages::created))
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
