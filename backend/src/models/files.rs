// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::schema::files;

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "files"]
pub struct NewFile {
    pub book: i32,
    pub format: i32,
    pub size: i32,
    pub name: String,
    pub sha: String,
}

#[derive(Debug, Serialize, Queryable)]
pub struct File {
    pub id: i32,
    pub book: i32,
    pub format: i32,
    pub size: i32,
    pub name: String,
    pub sha: String,
    pub created: NaiveDateTime,
    pub last_modified: NaiveDateTime,
}

pub fn add_file(conn: &PgConnection, new_file: &NewFile) -> Result<(), Error> {
    use crate::schema::files::dsl::files;
    diesel::insert_into(files).values(new_file).execute(conn)?;
    Ok(())
}

pub fn get_book_files(conn: &PgConnection, book_id: i32) -> Result<Vec<File>, Error> {
    use crate::schema::files::dsl::{book, files};
    files
        .filter(book.eq(book_id))
        .load::<File>(conn)
        .map_err(Into::into)
}
