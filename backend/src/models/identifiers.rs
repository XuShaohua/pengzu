// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use diesel::{ExpressionMethods, Insertable, PgConnection, QueryDsl, RunQueryDsl};
use serde::Deserialize;
use shared::identifiers::Identifier;

use crate::error::Error;
use crate::schema::identifiers;

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = identifiers)]
pub struct NewIdentifier {
    pub book: i32,
    pub scheme: i32,
    pub value: String,
    pub url: Option<String>,
}

pub fn add_identifier(
    conn: &mut PgConnection,
    new_identifier: &NewIdentifier,
) -> Result<(), Error> {
    use crate::schema::identifiers::dsl::identifiers;
    diesel::insert_into(identifiers)
        .values(new_identifier)
        .execute(conn)?;
    Ok(())
}

pub fn get_identifiers_by_book(
    conn: &mut PgConnection,
    book_id: i32,
) -> Result<Vec<Identifier>, Error> {
    use crate::schema::identifiers::dsl::{book, identifiers};
    identifiers
        .filter(book.eq(book_id))
        .load::<Identifier>(conn)
        .map_err(Into::into)
}
