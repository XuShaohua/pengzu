// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, QueryDsl, Queryable, RunQueryDsl, SqliteConnection};

use crate::error::Error;

#[derive(Debug, Clone, Queryable)]
pub struct CalibreBook {
    pub id: i32,
    pub title: String,
    pub sort: Option<String>,
    pub timestamp: Option<NaiveDateTime>,
    pub pubdate: Option<NaiveDateTime>,
    pub series_index: f32,
    pub author_sort: Option<String>,
    pub isbn: Option<String>,
    pub lccn: Option<String>,
    pub path: String,
    pub flags: i32,
    pub uuid: String,
    pub has_cover: bool,
    pub last_modified: NaiveDateTime,
}

pub fn get_next_book(conn: &SqliteConnection, book_id: i32) -> Result<CalibreBook, Error> {
    use crate::schema::books::dsl::{books, id};
    books
        .filter(id.gt(book_id))
        .order(id.asc())
        .first::<CalibreBook>(conn)
        .map_err(Into::into)
}
