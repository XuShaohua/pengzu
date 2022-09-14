// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SqliteConnection};

use crate::error::Error;

#[derive(Debug, Queryable)]
pub struct BookSeries {
    pub book: i32,
    pub name: String,
}

pub fn get_book_series(conn: &mut SqliteConnection, book_id: i32) -> Result<BookSeries, Error> {
    use crate::schema::{books_series_link, series};

    books_series_link::dsl::books_series_link
        .filter(books_series_link::book.eq(book_id))
        .inner_join(series::table.on(books_series_link::series.eq(series::id)))
        .select((books_series_link::book, series::name))
        .first::<BookSeries>(conn)
        .map_err(Into::into)
}
