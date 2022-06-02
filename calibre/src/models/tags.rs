// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use crate::error::Error;
use diesel::{QueryDsl, Queryable, RunQueryDsl, SqliteConnection};

#[derive(Debug, Queryable)]
pub struct Tag {
    pub id: i32,
    pub name: String,
}

pub fn get_tags(conn: &SqliteConnection, offset: i64) -> Result<Vec<Tag>, Error> {
    use crate::schema::tags::dsl::tags;
    tags.offset(offset).load::<Tag>(conn).map_err(Into::into)
}
