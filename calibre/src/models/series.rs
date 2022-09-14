// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use crate::error::Error;
use diesel::{QueryDsl, Queryable, RunQueryDsl, SqliteConnection};

#[derive(Debug, Queryable)]
pub struct Series {
    pub id: i32,
    pub name: String,
    pub sort: Option<String>,
}

pub fn get_series(
    conn: &mut SqliteConnection,
    limit: i64,
    offset: i64,
) -> Result<Vec<Series>, Error> {
    use crate::schema::series::dsl::series;
    series
        .limit(limit)
        .offset(offset)
        .load::<Series>(conn)
        .map_err(Into::into)
}
