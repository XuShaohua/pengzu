// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use diesel::{Insertable, PgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::schema::languages;

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "languages"]
pub struct NewLanguage {
    pub lang_code: String,
}

#[derive(Debug, Serialize, Queryable)]
pub struct Language {
    pub id: i32,
    pub lang_code: String,
}

pub fn add_language(conn: &PgConnection, new_language: &NewLanguage) -> Result<(), Error> {
    use crate::schema::languages::dsl::languages;
    diesel::insert_into(languages)
        .values(new_language)
        .execute(conn)?;
    Ok(())
}

pub fn get_all_languages(conn: &PgConnection) -> Result<Vec<Language>, Error> {
    use crate::schema::languages::dsl::languages;
    languages.load::<Language>(conn).map_err(Into::into)
}
