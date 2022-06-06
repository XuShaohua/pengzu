// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use diesel::{QueryDsl, Queryable, RunQueryDsl, SqliteConnection};

use crate::error::Error;

#[derive(Debug, Queryable)]
pub struct IdentifierType {
    pub format: String,
}

pub fn get_identifier_types(conn: &SqliteConnection) -> Result<Vec<IdentifierType>, Error> {
    use crate::schema::identifiers::dsl::{identifiers, type_};
    identifiers
        .distinct_on(type_)
        .select((type_,))
        .load::<IdentifierType>(conn)
        .map_err(Into::into)
}
