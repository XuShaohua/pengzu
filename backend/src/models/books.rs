// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use chrono::NaiveDateTime;
use diesel::{Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::schema::books;

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "books"]
pub struct NewBook {
    pub title: String,
    pub sort: String,
    pub author_sort: String,
    pub path: String,
    pub uuid: String,
    pub has_cover: bool,
}

#[derive(Debug, Serialize, Queryable)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub sort: String,
    pub author_sort: String,
    pub path: String,
    pub uuid: String,
    pub has_cover: bool,
    pub created: NaiveDateTime,
    pub pubdate: NaiveDateTime,
    pub last_modified: NaiveDateTime,
}

pub fn add_book(conn: &PgConnection, new_book: &NewBook) -> Result<Book, Error> {
    use crate::schema::books::dsl::books;
    diesel::insert_into(books)
        .values(new_book)
        .get_result::<Book>(conn)
        .map_err(Into::into)
}

pub fn get_book(conn: &PgConnection, book_id: i32) -> Result<Book, Error> {
    use crate::schema::books::dsl::books;
    books.find(book_id).first::<Book>(conn).map_err(Into::into)
}
