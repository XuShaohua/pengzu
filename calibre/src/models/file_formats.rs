// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use diesel::{QueryDsl, Queryable, RunQueryDsl, SqliteConnection};

use crate::error::Error;

#[derive(Debug, Queryable)]
pub struct FileFormat {
    pub format: String,
}

pub fn get_file_formats(conn: &SqliteConnection) -> Result<Vec<FileFormat>, Error> {
    use crate::schema::data::dsl::{data, format};
    data.distinct_on(format)
        .select((format,))
        .load::<FileFormat>(conn)
        .map_err(Into::into)
}
