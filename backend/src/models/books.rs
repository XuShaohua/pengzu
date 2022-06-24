// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use chrono::NaiveDateTime;
use diesel::{Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::models::file_data;
use crate::schema::books;

#[derive(Debug, Serialize, Queryable)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub path: String,
    pub uuid: String,
    pub has_cover: bool,
    pub created: NaiveDateTime,
    pub pubdate: NaiveDateTime,
    pub last_modified: NaiveDateTime,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "books"]
pub struct NewBook {
    pub title: String,
    pub path: String,
    pub uuid: String,
    pub has_cover: bool,
}

pub fn add_book(conn: &PgConnection, new_book: &NewBook) -> Result<Book, Error> {
    use crate::schema::books::dsl::books;
    diesel::insert_into(books)
        .values(new_book)
        .get_result::<Book>(conn)
        .map_err(Into::into)
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetBooksQuery {
    pub page: Option<i64>,
    pub sort: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct BookResp {
    pub id: i32,
    pub title: String,
    pub uuid: String,
    pub has_cover: bool,
    pub small_cover: Option<String>,
    pub large_cover: Option<String>,
    pub created: NaiveDateTime,
    pub pubdate: NaiveDateTime,
}

fn book_to_book_resp(book: Book) -> BookResp {
    BookResp {
        id: book.id,
        title: book.title,
        uuid: book.uuid,
        has_cover: book.has_cover,
        small_cover: file_data::get_small_cover(&book.path, book.has_cover),
        large_cover: file_data::get_large_cover(&book.path, book.has_cover),
        created: book.created,
        pubdate: book.pubdate,
    }
}

pub fn get_books(conn: &PgConnection, query: &GetBooksQuery) -> Result<Vec<BookResp>, Error> {
    use crate::schema::books::dsl::books;

    let page_id = if let Some(page) = query.page {
        if page < 0 {
            0
        } else {
            page
        }
    } else {
        0
    };
    let each_page = 20_i64;
    let offset = page_id * each_page;
    books
        .limit(each_page)
        .offset(offset)
        .load::<Book>(conn)
        .map(|list| list.into_iter().map(book_to_book_resp).collect())
        .map_err(Into::into)
}

pub fn get_book_detail(conn: &PgConnection, book_id: i32) -> Result<Book, Error> {
    use crate::schema::books::dsl::books;
    books.find(book_id).first::<Book>(conn).map_err(Into::into)
}
