// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::schema::identifier_types;

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = identifier_types)]
pub struct NewIdentifierType {
    pub name: String,
}

#[derive(Debug, Serialize, Queryable)]
pub struct IdentifierType {
    pub id: i32,
    pub name: String,
    pub url_template: String,
    pub description: Option<String>,
    pub crated: NaiveDateTime,
    pub last_modified: NaiveDateTime,
}

pub fn add_identifier_type(
    conn: &mut PgConnection,
    new_type: &NewIdentifierType,
) -> Result<(), Error> {
    diesel::insert_into(identifier_types::table)
        .values(new_type)
        .execute(conn)?;
    Ok(())
}

pub fn get_identifier_type_by_name(
    conn: &mut PgConnection,
    type_name: &str,
) -> Result<IdentifierType, Error> {
    identifier_types::table
        .filter(identifier_types::name.eq(type_name))
        .first(conn)
        .map_err(Into::into)
}

pub fn get_identifier_type(conn: &mut PgConnection, type_id: i32) -> Result<IdentifierType, Error> {
    identifier_types::table
        .find(type_id)
        .first(conn)
        .map_err(Into::into)
}
