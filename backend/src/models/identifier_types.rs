// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use chrono::NaiveDateTime;
use diesel::{Insertable, PgConnection, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::schema::identifier_types;

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "identifier_types"]
pub struct NewIdentifierType {
    pub name: String,
}

#[derive(Debug, Serialize, Queryable)]
pub struct IdentifierType {
    pub id: i32,
    pub name: String,
    pub crated: NaiveDateTime,
    pub last_modified: NaiveDateTime,
}

pub fn add_identifier_type(conn: &PgConnection, new_type: &NewIdentifierType) -> Result<(), Error> {
    use crate::schema::identifier_types::dsl::identifier_types;
    diesel::insert_into(identifier_types)
        .values(new_type)
        .execute(conn)?;
    Ok(())
}

pub fn get_file_format(conn: &PgConnection, type_id: i32) -> Result<IdentifierType, Error> {
    use crate::schema::identifier_types::dsl::identifier_types;
    identifier_types
        .find(type_id)
        .first(conn)
        .map_err(Into::into)
}
