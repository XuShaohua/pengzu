// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use diesel::{QueryDsl, Queryable, RunQueryDsl, SqliteConnection};

use crate::error::Error;

#[derive(Debug, Queryable)]
pub struct Author {
    pub id: i32,
    pub name: String,
    pub sort: Option<String>,
    pub link: String,
}

pub fn get_authors(conn: &SqliteConnection, offset: i64) -> Result<Vec<Author>, Error> {
    use crate::schema::authors::dsl::authors;
    authors
        .offset(offset)
        .load::<Author>(conn)
        .map_err(Into::into)
}
