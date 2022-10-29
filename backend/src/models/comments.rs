// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::schema::comments;

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = comments)]
pub struct NewComment {
    pub book: i32,
    pub text: String,
}

#[derive(Debug, Serialize, Queryable)]
pub struct Comment {
    pub id: i32,
    pub book: i32,
    pub text: String,
    pub created: NaiveDateTime,
    pub last_modified: NaiveDateTime,
}

pub fn add_comment(conn: &mut PgConnection, new_comment: &NewComment) -> Result<(), Error> {
    use crate::schema::comments::dsl::comments;
    diesel::insert_into(comments)
        .values(new_comment)
        .execute(conn)?;
    Ok(())
}

pub fn get_comment(conn: &mut PgConnection, book_id: i32) -> Result<Comment, Error> {
    use crate::schema::comments::dsl::{book, comments};
    comments
        .filter(book.eq(book_id))
        .first::<Comment>(conn)
        .map_err(Into::into)
}

pub fn update_comment(conn: &mut PgConnection, new_comment: &NewComment) -> Result<(), Error> {
    use crate::schema::comments::dsl::{book, comments, text};
    diesel::update(comments.filter(book.eq(new_comment.book)))
        .set(text.eq(new_comment.text.clone()))
        .execute(conn)?;
    Ok(())
}

pub fn delete_comment(conn: &mut PgConnection, book_id: i32) -> Result<(), Error> {
    use crate::schema::comments::dsl::{book, comments};
    let _comment = get_comment(conn, book_id)?;
    diesel::delete(comments.filter(book.eq(book_id))).execute(conn)?;
    Ok(())
}
