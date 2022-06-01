// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::schema::publishers;

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "publishers"]
pub struct NewPublisher {
    pub name: String,
    pub sort: String,
}

#[derive(Debug, Serialize, Queryable)]
pub struct Publisher {
    pub id: i32,
    pub name: String,
    pub sort: String,
    pub crated: NaiveDateTime,
    pub last_modified: NaiveDateTime,
}

pub fn add_publisher(conn: &PgConnection, new_publisher: &NewPublisher) -> Result<(), Error> {
    use crate::schema::publishers::dsl::publishers;
    diesel::insert_into(publishers)
        .values(new_publisher)
        .execute(conn)?;
    Ok(())
}

pub fn get_publisher(conn: &PgConnection, id: i32) -> Result<Publisher, Error> {
    use crate::schema::publishers::dsl::publishers;
    publishers.find(id).first(conn).map_err(Into::into)
}

pub fn update_publisher(
    conn: &PgConnection,
    pub_id: i32,
    new_publisher: &NewPublisher,
) -> Result<(), Error> {
    use crate::schema::publishers::dsl::{name, publishers, sort};
    diesel::update(publishers.find(pub_id))
        .set((
            name.eq(new_publisher.name.as_str()),
            sort.eq(new_publisher.sort.as_str()),
        ))
        .execute(conn)?;
    Ok(())
}
