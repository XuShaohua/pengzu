// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};
use shared::books::{AuthorAndBookId, BookAndAuthors, BookAndAuthorsList, BookWithCover};
use shared::books_query::GetBooksQuery;
use shared::page::{Page, BOOKS_EACH_PAGE};

use crate::error::Error;
use crate::models::authors::get_authors_by_book_id;
use crate::models::file_data;
use crate::schema::books;

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = books)]
pub struct NewBook {
    pub title: String,
    pub path: String,
    pub author_sort: String,
    pub uuid: String,
    pub has_cover: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Queryable)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub path: String,
    pub author_sort: String,
    pub uuid: String,
    pub has_cover: bool,
    pub pubdate: Option<NaiveDateTime>,
    pub created: NaiveDateTime,
    pub last_modified: NaiveDateTime,
}

pub fn add_book(conn: &mut PgConnection, new_book: &NewBook) -> Result<Book, Error> {
    use crate::schema::books::dsl::books;
    diesel::insert_into(books)
        .values(new_book)
        .get_result::<Book>(conn)
        .map_err(Into::into)
}

pub fn get_book_by_id(conn: &mut PgConnection, book_id: i32) -> Result<BookWithCover, Error> {
    books::table
        .find(book_id)
        .first(conn)
        .map(book_to_book_cover)
        .map_err(Into::into)
}

pub fn get_book_path_by_id(conn: &mut PgConnection, book_id: i32) -> Result<String, Error> {
    books::table
        .find(book_id)
        .select(books::path)
        .first(conn)
        .map_err(Into::into)
}

#[must_use]
pub fn book_to_book_cover(book: Book) -> BookWithCover {
    BookWithCover {
        id: book.id,
        title: book.title,
        path: file_data::get_book_file(&book.path),
        has_cover: book.has_cover,
        small_cover: file_data::get_small_cover(&book.path, book.has_cover),
        large_cover: file_data::get_large_cover(&book.path, book.has_cover),
        created: book.created,
        pubdate: book.pubdate,
    }
}

#[must_use]
pub fn merge_books_and_authors(
    book_list: Vec<Book>,
    author_list: &[AuthorAndBookId],
) -> Vec<BookAndAuthors> {
    let mut list = Vec::with_capacity(book_list.len());

    for book in book_list {
        let authors = author_list
            .iter()
            .filter(|author| author.book == book.id)
            .map(Clone::clone)
            .collect();
        list.push(BookAndAuthors {
            book: book_to_book_cover(book),
            authors,
        });
    }

    list
}

pub fn get_books(
    conn: &mut PgConnection,
    query: &GetBooksQuery,
) -> Result<BookAndAuthorsList, Error> {
    use crate::schema::books::dsl::{books, id};

    let page_id = if query.page < 1 { 0 } else { query.page - 1 };
    let offset = page_id * BOOKS_EACH_PAGE;

    let book_list = books
        .order_by(id.asc())
        .limit(BOOKS_EACH_PAGE)
        .offset(offset)
        .load::<Book>(conn)?;
    let author_list = get_authors_by_book_id(conn, &book_list)?;
    let list = merge_books_and_authors(book_list, &author_list);

    let total = books.count().first(conn)?;

    Ok(BookAndAuthorsList {
        page: Page {
            page_num: page_id + 1,
            each_page: BOOKS_EACH_PAGE,
            total,
        },
        list,
    })
}

pub fn get_books_by_ids(
    conn: &mut PgConnection,
    query: &GetBooksQuery,
    book_ids: &[i32],
) -> Result<BookAndAuthorsList, Error> {
    let page_id = query.backend_page_id();
    let offset = page_id * BOOKS_EACH_PAGE;
    // let order_column = query.order.get_column();
    let total = book_ids.len() as i64;

    // TODO(Shaohua): Apply query order
    let book_list = books::table
        .filter(books::id.eq_any(book_ids))
        .order_by(books::id.asc())
        .limit(BOOKS_EACH_PAGE)
        .offset(offset)
        .load::<Book>(conn)?;
    let author_list = get_authors_by_book_id(conn, &book_list)?;
    let list = merge_books_and_authors(book_list, &author_list);

    Ok(BookAndAuthorsList {
        page: Page {
            page_num: page_id + 1,
            each_page: BOOKS_EACH_PAGE,
            total,
        },
        list,
    })
}
