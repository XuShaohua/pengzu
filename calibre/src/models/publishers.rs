// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use crate::error::Error;
use diesel::{QueryDsl, Queryable, RunQueryDsl, SqliteConnection};

#[derive(Debug, Queryable)]
pub struct Publisher {
    pub id: i32,
    pub name: String,
    pub sort: Option<String>,
}

pub fn get_publishers(conn: &SqliteConnection, offset: i64) -> Result<Vec<Publisher>, Error> {
    use crate::schema::publishers::dsl::publishers;
    publishers
        .offset(offset)
        .load::<Publisher>(conn)
        .map_err(Into::into)
}
