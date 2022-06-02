// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use crate::error::Error;
use diesel::{Queryable, RunQueryDsl, SqliteConnection};

#[derive(Debug, Queryable)]
pub struct Language {
    pub id: i32,
    pub lang_code: String,
}

pub fn get_languages(conn: &SqliteConnection) -> Result<Vec<Language>, Error> {
    use crate::schema::languages::dsl::languages;
    languages.load::<Language>(conn).map_err(Into::into)
}
