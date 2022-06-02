// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SqliteConnection};

use crate::error::Error;

#[derive(Debug, Queryable)]
pub struct BookLanguage {
    pub book: i32,
    pub lang_code: String,
}

pub fn get_book_language(conn: &SqliteConnection, book_id: i32) -> Result<BookLanguage, Error> {
    use crate::schema::{books_languages_link, languages};

    books_languages_link::dsl::books_languages_link
        .filter(books_languages_link::book.eq(book_id))
        .inner_join(languages::table.on(books_languages_link::lang_code.eq(languages::id)))
        .select((books_languages_link::book, languages::lang_code))
        .first::<BookLanguage>(conn)
        .map_err(Into::into)
}
