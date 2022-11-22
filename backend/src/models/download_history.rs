// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};
use shared::books::BookAndAuthorsList;
use shared::books_query::GetBooksQuery;

use crate::error::Error;
use crate::models::books::get_books_by_ids;
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
    // TODO(Shaohua): Apply query.
    let book_ids = download_history::table
        .filter(download_history::user_id.eq(user_id))
        .select(download_history::book)
        .order_by(download_history::id.desc())
        .load::<i32>(conn)?;

    get_books_by_ids(conn, query, &book_ids)
}
