// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};
use shared::books::BookAndAuthorsList;
use shared::books_query::GetBooksQuery;
use shared::page::BOOKS_EACH_PAGE;
use shared::publishers::Publisher;

use crate::error::{Error, ErrorKind};
use crate::models::books::{book_list_to_book_authors, Book};
use crate::models::books_query::sort_books_by_column;
use crate::schema::books_publishers_link;

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = books_publishers_link)]
pub struct NewBookPublisher {
    pub book: i32,
    pub publisher: i32,
}

#[derive(Debug, Serialize, Queryable)]
pub struct BookPublisher {
    pub id: i32,
    pub book: i32,
    pub publisher: i32,
    pub created: NaiveDateTime,
}

pub fn add_book_publisher(
    conn: &mut PgConnection,
    new_book_publisher: &NewBookPublisher,
) -> Result<(), Error> {
    diesel::insert_into(books_publishers_link::table)
        .values(new_book_publisher)
        .execute(conn)?;
    Ok(())
}

pub fn get_book_publisher(conn: &mut PgConnection, book_id: i32) -> Result<BookPublisher, Error> {
    books_publishers_link::table
        .filter(books_publishers_link::book.eq(book_id))
        .first::<BookPublisher>(conn)
        .map_err(Into::into)
}

pub fn delete_book_publisher(conn: &mut PgConnection, book_id: i32) -> Result<(), Error> {
    let _link = get_book_publisher(conn, book_id)?;
    diesel::delete(books_publishers_link::table)
        .filter(books_publishers_link::book.eq(book_id))
        .execute(conn)?;
    Ok(())
}

pub fn delete_by_publisher_id(conn: &mut PgConnection, publisher_id: i32) -> Result<(), Error> {
    diesel::delete(
        books_publishers_link::table.filter(books_publishers_link::publisher.eq(publisher_id)),
    )
    .execute(conn)?;
    Ok(())
}

pub fn get_publisher_by_book(
    conn: &mut PgConnection,
    book_id: i32,
) -> Result<Option<Publisher>, Error> {
    use crate::schema::publishers;

    let publisher: Result<Publisher, Error> = publishers::table
        .filter(
            publishers::id.eq_any(
                books_publishers_link::table
                    .filter(books_publishers_link::book.eq(book_id))
                    .select(books_publishers_link::publisher),
            ),
        )
        .first::<Publisher>(conn)
        .map_err(Into::into);

    match publisher {
        Ok(publisher) => Ok(Some(publisher)),
        Err(err) => match err.kind() {
            ErrorKind::DbNotFoundError => Ok(None),
            _ => Err(err),
        },
    }
}

pub fn get_books_by_publisher(
    conn: &mut PgConnection,
    publisher_id: i32,
    query: &GetBooksQuery,
) -> Result<BookAndAuthorsList, Error> {
    use crate::schema::books;

    let offset = query.backend_page_id() * BOOKS_EACH_PAGE;
    let total = books_publishers_link::table
        .filter(books_publishers_link::publisher.eq(publisher_id))
        .count()
        .first::<i64>(conn)?;

    // Get book list based on a subquery.
    let book_list = sort_books_by_column(query.order)
        .filter(
            books::id.eq_any(
                books_publishers_link::table
                    .filter(books_publishers_link::publisher.eq(publisher_id))
                    .select(books_publishers_link::book),
            ),
        )
        .limit(BOOKS_EACH_PAGE)
        .offset(offset)
        .load::<Book>(conn)?;

    book_list_to_book_authors(conn, book_list, query, total)
}
