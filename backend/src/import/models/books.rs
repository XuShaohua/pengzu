// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use crate::error::Error;
use crate::import::import_books::import_books;
use chrono::NaiveDateTime;
use diesel::{PgConnection, RunQueryDsl};

pub struct NewImportBook {
    pub library: i32,
    pub calibre_book: i32,
    pub ok: bool,
    pub book: Option<i32>,
}

pub struct ImportBook {
    pub id: i32,
    pub library: i32,
    pub calibre_book: i32,
    pub ok: bool,
    pub book: Option<i32>,
    pub created: NaiveDateTime,
}

pub fn add_import_book(conn: &PgConnection, new_book: &NewImportBook) -> Result<(), Error> {
    use crate::schema::import_books::dsl::import_books;
    diesel::insert_into(import_books)
        .values(new_book)
        .execute(conn)
        .map(drop)
        .map_err(Into::into)
}
