// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::schema::authors;

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "authors"]
pub struct NewAuthor {
    pub name: String,
    pub sort: String,
    pub link: String,
}

#[derive(Debug, Serialize, Queryable)]
pub struct Author {
    pub id: i32,
    pub name: String,
    pub sort: String,
    pub link: String,
    pub created: NaiveDateTime,
    pub last_modified: NaiveDateTime,
}

pub fn add_author(conn: &PgConnection, new_author: &NewAuthor) -> Result<(), Error> {
    use crate::schema::authors::dsl::authors;
    diesel::insert_into(authors)
        .values(new_author)
        .execute(conn)?;
    Ok(())
}

pub fn get_author_by_name(conn: &PgConnection, author_name: &str) -> Result<Author, Error> {
    use crate::schema::authors::dsl::{authors, name};
    authors
        .filter(name.eq(author_name))
        .first::<Author>(conn)
        .map_err(Into::into)
}

pub fn update_author(
    conn: &PgConnection,
    author_id: i32,
    new_author: &NewAuthor,
) -> Result<(), Error> {
    use crate::schema::authors::dsl::{authors, name, sort};
    diesel::update(authors.find(author_id))
        .set((
            name.eq(new_author.name.as_str()),
            sort.eq(new_author.sort.as_str()),
        ))
        .execute(conn)?;
    Ok(())
}
