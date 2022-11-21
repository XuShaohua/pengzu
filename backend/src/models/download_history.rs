// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use chrono::NaiveDateTime;
use diesel::{Insertable, PgConnection, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::schema::download_history;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Queryable)]
pub struct DownloadHistory {
    pub id: i32,
    pub user_id: i32,
    pub book: i32,
    pub file: i32,
    pub created: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = download_history)]
pub struct NewHistory {
    pub user_id: i32,
    pub book: i32,
    pub file: i32,
}

pub fn add(conn: &mut PgConnection, new_history: &NewHistory) -> Result<(), Error> {
    diesel::insert_into(download_history::table)
        .values(new_history)
        .execute(conn)
        .map(drop)
        .map_err(Into::into)
}
