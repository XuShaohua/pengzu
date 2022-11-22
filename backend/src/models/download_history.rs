// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use chrono::NaiveDateTime;
use diesel::{
    ExpressionMethods, Insertable, JoinOnDsl, PgConnection, QueryDsl, Queryable, RunQueryDsl,
};
use serde::{Deserialize, Serialize};
use shared::books::BookAndAuthorsList;
use shared::books_query::GetBooksQuery;
use shared::page::BOOKS_EACH_PAGE;

use crate::error::Error;
use crate::models::books::{book_list_to_book_authors, Book};
use crate::schema::download_history;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Queryable)]
pub struct DownloadHistory {
    pub id: i32,
    pub user_id: i32,
    pub book: i32,
    pub file: i32,
    pub created: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = download_history)]
pub struct NewHistory {
    pub user_id: i32,
    pub book: i32,
    pub file: i32,
}

pub fn add(conn: &mut PgConnection, new_history: &NewHistory) -> Result<(), Error> {
    diesel::insert_into(download_history::table)
        .values(new_history)
        .execute(conn)
        .map(drop)
        .map_err(Into::into)
}

pub fn get_books(
    conn: &mut PgConnection,
    user_id: i32,
    query: &GetBooksQuery,
) -> Result<BookAndAuthorsList, Error> {
    use crate::schema::books;

    let offset = query.backend_page_id() * BOOKS_EACH_PAGE;

    // TODO(Shaohua): Replace with `SELECT *`
    // -- get download books
    // SELECT books.* FROM books
    // INNER JOIN download_history
    //     ON download_history.book = books.id
    // WHERE download_history.user_id = 1
    // ORDER BY download_history.id DESC
    // LIMIT 50;
    let book_list = books::table
        .inner_join(download_history::table.on(download_history::book.eq(books::id)))
        .filter(download_history::user_id.eq(user_id))
        .order_by(download_history::id.desc())
        .select((
            books::id,
            books::title,
            books::path,
            books::author_sort,
            books::uuid,
            books::has_cover,
            books::pubdate,
            books::created,
            books::last_modified,
        ))
        .offset(offset)
        .limit(BOOKS_EACH_PAGE)
        .load::<Book>(conn)?;

    let total = download_history::table
        .filter(download_history::user_id.eq(user_id))
        .count()
        .first::<i64>(conn)?;

    book_list_to_book_authors(conn, book_list, query, total)
}
