// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use diesel::{QueryDsl, RunQueryDsl, SqliteConnection};

use crate::error::Error;

pub fn get_file_formats(conn: &mut SqliteConnection) -> Result<Vec<String>, Error> {
    use crate::schema::data::dsl::{data, format};
    data.select(format)
        .distinct()
        .load::<String>(conn)
        .map_err(Into::into)
}
