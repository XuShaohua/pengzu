// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use chrono::NaiveDateTime;
use diesel::{Insertable, PgConnection, Queryable, RunQueryDsl};
use serde::Serialize;

use crate::error::Error;
use crate::schema::import_libraries;

#[derive(Debug, Serialize, Insertable)]
#[table_name = "import_libraries"]
pub struct NewImportLibrary {
    pub calibre_library_path: String,
    pub library_path: String,
    pub total: i32,
    pub finished: bool,
}

#[derive(Debug, Queryable)]
pub struct ImportLibrary {
    pub id: i32,
    pub calibre_library_path: String,
    pub library_path: String,
    pub total: i32,
    pub finished: bool,
    pub worker_pid: Option<i32>,
    pub created: NaiveDateTime,
    pub last_modified: NaiveDateTime,
}

pub fn add_import_library(
    conn: &PgConnection,
    new_library: &NewImportLibrary,
) -> Result<ImportLibrary, Error> {
    use crate::schema::import_libraries::dsl::import_libraries;
    diesel::insert_into(import_libraries)
        .values(new_library)
        .get_result::<ImportLibrary>(conn)
        .map_err(Into::into)
}
