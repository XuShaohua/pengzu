// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use chrono::NaiveDateTime;
use diesel::{Insertable, PgConnection, Queryable, RunQueryDsl};
use serde::Serialize;

use crate::error::Error;
use crate::schema::import_books;

#[derive(Debug, Serialize, Insertable)]
#[diesel(table_name = import_books)]
pub struct NewImportBook {
    pub library: i32,
    pub calibre_book: i32,
    pub ok: bool,
    pub book: Option<i32>,
}

#[allow(dead_code)]
#[derive(Debug, Queryable)]
pub struct ImportBook {
    pub id: i32,
    pub library: i32,
    pub calibre_book: i32,
    pub ok: bool,
    pub book: Option<i32>,
    pub created: NaiveDateTime,
}

pub fn add_import_book(conn: &mut PgConnection, new_book: &NewImportBook) -> Result<(), Error> {
    use crate::schema::import_books::dsl::import_books;
    diesel::insert_into(import_books)
        .values(new_book)
        .execute(conn)
        .map(drop)
        .map_err(Into::into)
}
