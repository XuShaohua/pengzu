// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, Insertable, PgConnection, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::schema::file_formats;

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "file_formats"]
pub struct NewFileFormat {
    pub name: String,
}

#[derive(Debug, Serialize, Queryable)]
pub struct FileFormat {
    pub id: i32,
    pub name: String,
    pub crated: NaiveDateTime,
    pub last_modified: NaiveDateTime,
}

pub fn add_file_format(conn: &PgConnection, new_format: &NewFileFormat) -> Result<(), Error> {
    use crate::schema::file_formats::dsl::file_formats;
    diesel::insert_into(file_formats)
        .values(new_format)
        .execute(conn)?;
    Ok(())
}

pub fn get_file_format_by_name(
    conn: &PgConnection,
    format_name: &str,
) -> Result<FileFormat, Error> {
    use crate::schema::file_formats::dsl::{file_formats, name};
    file_formats
        .filter(name.eq(format_name))
        .first(conn)
        .map_err(Into::into)
}

pub fn get_file_format(conn: &PgConnection, format_id: i32) -> Result<FileFormat, Error> {
    use crate::schema::file_formats::dsl::file_formats;
    file_formats.find(format_id).first(conn).map_err(Into::into)
}
