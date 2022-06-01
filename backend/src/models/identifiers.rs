// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use chrono::NaiveDateTime;
use diesel::{EqAll, Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::schema::identifiers;

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "identifiers"]
pub struct NewIdentifier {
    pub book: i32,
    pub scheme: i32,
    pub url: Option<String>,
}

#[derive(Debug, Serialize, Queryable)]
pub struct Identifier {
    pub id: i32,
    pub book: i32,
    pub scheme: i32,
    pub value: String,
    pub url: Option<String>,
    pub created: NaiveDateTime,
    pub last_modified: NaiveDateTime,
}

pub fn add_identifier(conn: &PgConnection, new_identifier: &NewIdentifier) -> Result<(), Error> {
    use crate::schema::identifiers::dsl::identifiers;
    diesel::insert_into(identifiers)
        .values(new_identifier)
        .execute(conn)?;
    Ok(())
}

pub fn get_identifiers(conn: &PgConnection, book_id: i32) -> Result<Vec<Identifier>, Error> {
    use crate::schema::identifiers::dsl::{book, identifiers};
    identifiers
        .filter(book.eq_all(book_id))
        .load::<Identifier>(conn)
        .map_err(Into::into)
}
