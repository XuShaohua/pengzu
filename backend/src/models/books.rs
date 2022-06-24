// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use chrono::NaiveDateTime;
use diesel::{Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::models::{common_page, file_data};
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

#[derive(Debug, Clone, Copy, Deserialize)]
pub enum GetBooksOrder {
    Id,
    IdInc,
    Created,
    CreatedInc,
    LastModified,
    LastModifiedInc,
    Pubdate,
    PubdateInc,
}

impl Default for GetBooksOrder {
    fn default() -> Self {
        Self::Pubdate
    }
}

#[must_use]
const fn default_page_id() -> i64 {
    0
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetBooksQuery {
    #[serde(default = "default_page_id")]
    pub page: i64,
    #[serde(default = "GetBooksOrder::default")]
    pub order: GetBooksOrder,
}

#[derive(Debug, Clone, Serialize)]
pub struct BookResp {
    pub id: i32,
    pub title: String,
    pub has_cover: bool,
    pub small_cover: Option<String>,
    pub large_cover: Option<String>,
    pub created: NaiveDateTime,
    pub pubdate: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize)]
pub struct GetBooksResp {
    pub page: common_page::Page,
    pub list: Vec<BookResp>,
}

fn book_to_book_resp(book: Book) -> BookResp {
    BookResp {
        id: book.id,
        title: book.title,
        has_cover: book.has_cover,
        small_cover: file_data::get_small_cover(&book.path, book.has_cover),
        large_cover: file_data::get_large_cover(&book.path, book.has_cover),
        created: book.created,
        pubdate: book.pubdate,
    }
}

pub fn get_books(conn: &PgConnection, query: &GetBooksQuery) -> Result<GetBooksResp, Error> {
    use crate::schema::books::dsl::books;

    log::info!("query: {:?}", query);

    let page_id = if query.page < 1 { 0 } else { query.page - 1 };
    let each_page = 20_i64;
    let offset = page_id * each_page;
    let book_list = books.limit(each_page).offset(offset).load::<Book>(conn)?;
    let book_list = book_list.into_iter().map(book_to_book_resp).collect();

    let total = books.count().first(conn)?;
    Ok(GetBooksResp {
        page: common_page::Page {
            page_num: page_id + 1,
            each_page,
            total,
        },
        list: book_list,
    })
}

pub fn get_book_detail(conn: &PgConnection, book_id: i32) -> Result<Book, Error> {
    use crate::schema::books::dsl::books;
    books.find(book_id).first::<Book>(conn).map_err(Into::into)
}
