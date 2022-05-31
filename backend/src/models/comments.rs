// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use diesel::{PgConnection, RunQueryDsl};
use serde::Deserialize;

use crate::error::Error;
use crate::schema::comments;

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "comments"]
pub struct NewComment {
    book: i32,
    text: String,
}

pub fn add_comment(conn: &PgConnection, new_comment: &NewComment) -> Result<(), Error> {
    use crate::schema::comments::dsl::*;

    diesel::insert_into(comments)
        .values(new_comment)
        .execute(conn)?;
    Ok(())
}

pub fn get_comments() {
    todo!();
}

pub fn update_comment() {
    todo!();
}

pub fn delete_comment() {
    todo!();
}
