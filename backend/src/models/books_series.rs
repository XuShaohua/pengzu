// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};
use shared::books::BookAndAuthorsList;
use shared::books_query::GetBooksQuery;
use shared::page::BOOKS_EACH_PAGE;
use shared::series::Series;

use crate::error::{Error, ErrorKind};
use crate::models::books::{book_list_to_book_authors, Book};
use crate::models::books_query::sort_books_by_column;
use crate::schema::books_series_link;

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = books_series_link)]
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

pub fn add_book_series(
    conn: &mut PgConnection,
    new_book_series: &NewBookSeries,
) -> Result<(), Error> {
    diesel::insert_into(books_series_link::table)
        .values(new_book_series)
        .execute(conn)?;
    Ok(())
}

pub fn get_book_series(conn: &mut PgConnection, book_id: i32) -> Result<BookSeries, Error> {
    books_series_link::table
        .filter(books_series_link::book.eq(book_id))
        .first::<BookSeries>(conn)
        .map_err(Into::into)
}

pub fn delete_book_series(conn: &mut PgConnection, book_id: i32) -> Result<(), Error> {
    let _link = get_book_series(conn, book_id)?;
    diesel::delete(books_series_link::table)
        .filter(books_series_link::book.eq(book_id))
        .execute(conn)?;
    Ok(())
}

pub fn delete_by_series_id(conn: &mut PgConnection, series_id: i32) -> Result<(), Error> {
    diesel::delete(books_series_link::table.filter(books_series_link::series.eq(series_id)))
        .execute(conn)?;
    Ok(())
}

pub fn get_series_by_book(conn: &mut PgConnection, book_id: i32) -> Result<Option<Series>, Error> {
    use crate::schema::series;

    let series: Result<Series, Error> = series::table
        .filter(
            series::id.eq_any(
                books_series_link::table
                    .filter(books_series_link::book.eq(book_id))
                    .select(books_series_link::series),
            ),
        )
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

pub fn get_books_by_series(
    conn: &mut PgConnection,
    series_id: i32,
    query: &GetBooksQuery,
) -> Result<BookAndAuthorsList, Error> {
    use crate::schema::books;

    let offset = query.backend_page_id() * BOOKS_EACH_PAGE;
    let total = books_series_link::table
        .filter(books_series_link::series.eq(series_id))
        .count()
        .first::<i64>(conn)?;

    // Get book list based on a subquery.
    let book_list = sort_books_by_column(query.order)
        .filter(
            books::id.eq_any(
                books_series_link::table
                    .filter(books_series_link::series.eq(series_id))
                    .select(books_series_link::book),
            ),
        )
        .limit(BOOKS_EACH_PAGE)
        .offset(offset)
        .load::<Book>(conn)?;

    book_list_to_book_authors(conn, book_list, query, total)
}
