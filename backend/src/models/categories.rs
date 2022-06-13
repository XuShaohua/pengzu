// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::schema::categories;

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "categories"]
pub struct NewCategory<'a> {
    pub order_index: i32,
    pub serial_number: &'a str,
    pub name: &'a str,
    pub url: &'a str,
    pub description: Option<&'a str>,
    pub parent: i32,
}

#[derive(Debug, Serialize, Queryable)]
pub struct Category {
    pub id: i32,
    pub order_index: i32,
    pub serial_number: String,
    pub name: String,
    pub url: String,
    pub description: Option<String>,
    pub parent: i32,
    pub created: NaiveDateTime,
    pub last_modified: NaiveDateTime,
}

pub fn add_category(conn: &PgConnection, new_category: &NewCategory) -> Result<(), Error> {
    use crate::schema::categories::dsl::categories;
    diesel::insert_into(categories)
        .values(new_category)
        .execute(conn)?;
    Ok(())
}

pub fn get_category_by_serial_number(
    conn: &PgConnection,
    serial_number_val: &str,
) -> Result<Category, Error> {
    use crate::schema::categories::dsl::{categories, serial_number};
    categories
        .filter(serial_number.eq(serial_number_val))
        .first(conn)
        .map_err(Into::into)
}
