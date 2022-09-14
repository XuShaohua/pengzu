// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use diesel::{ExpressionMethods, QueryDsl, Queryable, RunQueryDsl, SqliteConnection};

use crate::error::Error;

#[derive(Debug, Queryable)]
pub struct Identifier {
    pub id: i32,
    pub book: i32,
    pub type_: String,
    pub val: String,
}

pub fn get_identifiers(
    conn: &mut SqliteConnection,
    book_id: i32,
) -> Result<Vec<Identifier>, Error> {
    use crate::schema::identifiers::dsl::{book, identifiers};
    identifiers
        .filter(book.eq(book_id))
        .load::<Identifier>(conn)
        .map_err(Into::into)
}
