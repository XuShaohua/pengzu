// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use diesel::{ExpressionMethods, Insertable, JoinOnDsl, PgConnection, QueryDsl, RunQueryDsl};
use serde::Deserialize;
use shared::identifiers::{Identifier, IdentifierWithType};

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
    diesel::insert_into(identifiers::table)
        .values(new_identifier)
        .execute(conn)?;
    Ok(())
}

pub fn get_identifiers(conn: &mut PgConnection, book_id: i32) -> Result<Vec<Identifier>, Error> {
    identifiers::table
        .filter(identifiers::book.eq(book_id))
        .load::<Identifier>(conn)
        .map_err(Into::into)
}

pub fn get_identifiers_with_type(
    conn: &mut PgConnection,
    book_id: i32,
) -> Result<Vec<IdentifierWithType>, Error> {
    use crate::schema::identifier_types;
    identifiers::table
        .inner_join(identifier_types::table.on(identifiers::scheme.eq(identifier_types::id)))
        .select((
            identifiers::id,
            identifiers::book,
            identifiers::scheme,
            identifiers::value,
            identifiers::created,
            identifiers::last_modified,
            identifier_types::name,
            identifier_types::url_template,
        ))
        .filter(identifiers::book.eq(book_id))
        .load::<IdentifierWithType>(conn)
        .map_err(Into::into)
}
