// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, PgConnection, QueryDsl, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};
use shared::books_query::GetBooksQuery;
use shared::page::{Page, READING_HISTORY_EACH_PAGE};

use crate::error::Error;
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

pub fn get_history_list(
    conn: &mut PgConnection,
    query: &GetBooksQuery,
) -> Result<GetHistoryResp, Error> {
    let offset = query.backend_page_id() * READING_HISTORY_EACH_PAGE;

    let list = reading_history::table
        .limit(READING_HISTORY_EACH_PAGE)
        .offset(offset)
        .load::<History>(conn)?;

    let total = reading_history::table.count().first(conn)?;

    Ok(GetHistoryResp {
        page: Page {
            page_num: query.frontend_page_id(),
            each_page: READING_HISTORY_EACH_PAGE,
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
