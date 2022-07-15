// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use chrono::NaiveDateTime;
use diesel::{
    ExpressionMethods, Insertable, JoinOnDsl, PgConnection, QueryDsl, Queryable, RunQueryDsl,
};
use serde::{Deserialize, Serialize};

use crate::error::{Error, ErrorKind};
use crate::models::series::Series;
use crate::schema::books_series_link;

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "books_series_link"]
pub struct NewBookSeries {
    pub book: i32,
    pub series: i32,
}

#[derive(Debug, Serialize, Queryable)]
pub struct BookSeries {
    pub id: i32,
    pub book: i32,
    pub series: i32,
    pub created: NaiveDateTime,
}

pub fn add_book_series(conn: &PgConnection, new_book_series: &NewBookSeries) -> Result<(), Error> {
    use crate::schema::books_series_link::dsl::books_series_link;
    diesel::insert_into(books_series_link)
        .values(new_book_series)
        .execute(conn)?;
    Ok(())
}

pub fn get_book_series(conn: &PgConnection, book_id: i32) -> Result<BookSeries, Error> {
    use crate::schema::books_series_link::dsl::{book, books_series_link};
    books_series_link
        .filter(book.eq(book_id))
        .first::<BookSeries>(conn)
        .map_err(Into::into)
}

pub fn delete_book_series(conn: &PgConnection, book_id: i32) -> Result<(), Error> {
    use crate::schema::books_series_link::dsl::{book, books_series_link};
    let _link = get_book_series(conn, book_id)?;
    diesel::delete(books_series_link)
        .filter(book.eq(book_id))
        .execute(conn)?;
    Ok(())
}

pub fn get_series_by_book(conn: &PgConnection, book_id: i32) -> Result<Option<Series>, Error> {
    use crate::schema::series;

    let series: Result<Series, Error> = series::table
        .inner_join(books_series_link::table.on(books_series_link::series.eq(series::id)))
        .filter(books_series_link::book.eq(book_id))
        .select((
            series::id,
            series::name,
            series::created,
            series::last_modified,
        ))
        .first::<Series>(conn)
        .map_err(Into::into);

    match series {
        Ok(series) => Ok(Some(series)),
        Err(err) => match err.kind() {
            ErrorKind::DbNotFoundError => Ok(None),
            _ => Err(err),
        },
    }
}
