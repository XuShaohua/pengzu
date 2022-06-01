// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::schema::tags;

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "tags"]
pub struct NewTag {
    pub name: String,
}

#[derive(Debug, Serialize, Queryable)]
pub struct Tag {
    pub id: i32,
    pub name: String,
    pub created: NaiveDateTime,
    pub last_modified: NaiveDateTime,
}

pub fn add_tag(conn: &PgConnection, new_tag: &NewTag) -> Result<(), Error> {
    use crate::schema::tags::dsl::tags;
    diesel::insert_into(tags).values(new_tag).execute(conn)?;
    Ok(())
}

pub fn get_all_tags(conn: &PgConnection) -> Result<Vec<Tag>, Error> {
    use crate::schema::tags::dsl::tags;
    tags.load::<Tag>(conn).map_err(Into::into)
}

pub fn update_tag(conn: &PgConnection, tag_id: i32, new_tag: &NewTag) -> Result<(), Error> {
    use crate::schema::tags::dsl::{name, tags};
    diesel::update(tags.find(tag_id))
        .set(name.eq(new_tag.name.as_str()))
        .execute(conn)?;
    Ok(())
}
