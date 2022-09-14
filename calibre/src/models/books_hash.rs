// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SqliteConnection};
use serde::Deserialize;
use std::collections::HashMap;

use crate::error::Error;

#[derive(Debug, Deserialize)]
pub struct FindDuplicateItem {
    pub sha: String,
    pub size: i32,
}

pub type FindDuplicateMap = HashMap<String, FindDuplicateItem>;

pub fn get_book_hash(conn: &mut SqliteConnection, book_id: i32) -> Result<FindDuplicateMap, Error> {
    use crate::schema::books_plugin_data::dsl::{book, books_plugin_data, name, val};
    let duplicate_str = books_plugin_data
        .filter(book.eq(book_id))
        .filter(name.eq("find_duplicates"))
        .select(val)
        .first::<String>(conn)?;
    serde_json::from_str(&duplicate_str).map_err(Into::into)
}
