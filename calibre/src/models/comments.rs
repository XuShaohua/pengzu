// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SqliteConnection};

use crate::error::Error;

#[derive(Debug, Queryable)]
pub struct Comment {
    pub id: i32,
    pub book: i32,
    pub text: String,
}

pub fn get_comment(conn: &SqliteConnection, book_id: i32) -> Result<Comment, Error> {
    use crate::schema::comments::dsl::{book, comments};
    comments
        .filter(book.eq(book_id))
        .first::<Comment>(conn)
        .map_err(Into::into)
}
