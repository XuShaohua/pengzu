// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use chrono::NaiveDateTime;
use diesel::{EqAll, PgConnection, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::schema::comments;

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "comments"]
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

pub fn add_comment(conn: &PgConnection, new_comment: &NewComment) -> Result<(), Error> {
    use crate::schema::comments::dsl::*;

    diesel::insert_into(comments)
        .values(new_comment)
        .execute(conn)?;
    Ok(())
}

pub fn get_comment(conn: &PgConnection, book_id: i32) -> Result<Comment, Error> {
    use crate::schema::comments::dsl::*;
    let resp_comment = comments
        .filter(book.eq_all(book_id))
        .limit(1)
        .first::<Comment>(conn)?;
    Ok(resp_comment)
}

pub fn update_comment(conn: &PgConnection, new_comment: &NewComment) -> Result<(), Error> {
    use crate::schema::comments::dsl::*;
    diesel::update(comments.filter(book.eq_all(new_comment.book)))
        .set(text.eq_all(new_comment.text.clone()))
        .execute(conn)?;
    Ok(())
}

pub fn delete_comment(conn: &PgConnection, book_id: i32) -> Result<(), Error> {
    use crate::schema::comments::dsl::*;
    let _comment = get_comment(conn, book_id)?;
    diesel::delete(comments.filter(book.eq_all(book_id))).execute(conn)?;
    Ok(())
}
