// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use diesel::{PgConnection, PgTextExpressionMethods, QueryDsl, RunQueryDsl};
use serde::Deserialize;
use shared::page::{default_page_id, PageId};

use crate::error::Error;
use crate::models::books::{get_books_by_ids, GetBooksResp};
use crate::models::books_query::{GetBooksOrder, GetBooksQuery};

#[derive(Debug, Clone, Deserialize)]
pub struct SimpleSearchQuery {
    #[serde(default = "default_page_id")]
    pub page: PageId,
    #[serde(default = "GetBooksOrder::default")]
    pub order: GetBooksOrder,

    pub query: String,
}

pub fn get_books_by_simple_search(
    conn: &mut PgConnection,
    query: &SimpleSearchQuery,
) -> Result<GetBooksResp, Error> {
    use crate::schema::books;

    let query_pattern = format!("%{}%", query.query);
    let books_query = GetBooksQuery {
        page: query.page,
        order: query.order,
    };

    let book_ids = books::table
        .filter(books::title.ilike(&query_pattern))
        .select(books::id)
        .load::<i32>(conn)?;

    get_books_by_ids(conn, &books_query, &book_ids)
}
