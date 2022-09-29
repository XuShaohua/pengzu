// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::models::page::{default_page_id, Page};
use crate::schema::reading_history;

#[derive(Debug, Serialize, Queryable)]
pub struct History {
    pub id: i32,
    pub user_id: i32,
    pub book: i32,
    pub page: i32,
    pub percent: i32,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct GetHistoryResp {
    pub page: Page,
    pub list: Vec<History>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GetHistoryReq {
    pub user_id: i32,
    #[serde(default = "default_page_id")]
    pub page: i64,
}

pub fn get_history_list(
    conn: &mut PgConnection,
    query: &GetHistoryReq,
) -> Result<GetHistoryResp, Error> {
    let page_id = if query.page < 1 { 0 } else { query.page - 1 };
    let each_page = 100;
    let offset = page_id * each_page;

    let list = reading_history::table
        .limit(each_page)
        .offset(offset)
        .load::<History>(conn)?;

    let total = reading_history::table.count().first(conn)?;

    Ok(GetHistoryResp {
        page: Page {
            page_num: page_id + 1,
            each_page,
            total,
        },
        list,
    })
}

#[derive(Debug, Deserialize)]
pub struct NewHistory {
    pub book_id: i32,
    pub page: i32,
    pub percent: i32,
}

pub fn update_history(
    conn: &mut PgConnection,
    history_id: i32,
    new_history: &NewHistory,
) -> Result<(), Error> {
    // TODO(Shaohua): Add user_id to parameter.
    // TODO(Shaohua): Replace with insert_or_update
    diesel::update(reading_history::table.find(history_id))
        .set(reading_history::percent.eq(new_history.percent))
        .execute(conn)?;
    Ok(())
}
