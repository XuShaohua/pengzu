// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use diesel::{ExpressionMethods, QueryDsl, Queryable, RunQueryDsl, SqliteConnection};

use crate::error::Error;

#[derive(Debug, Queryable)]
pub struct Data {
    pub id: i32,
    pub book: i32,
    pub format: String,
    pub uncompressed_size: i32,
    pub name: String,
}

pub fn get_book_data(conn: &SqliteConnection, book_id: i32) -> Result<Vec<Data>, Error> {
    use crate::schema::data::dsl::{book, data};
    data.filter(book.eq(book_id)).load(conn).map_err(Into::into)
}
