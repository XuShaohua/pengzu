// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use diesel::{PgConnection, PgTextExpressionMethods, QueryDsl, RunQueryDsl};
use serde::Deserialize;

use crate::error::Error;
use crate::models::books::{get_books_by_ids, GetBooksOrder, GetBooksQuery, GetBooksResp};
use crate::models::page::{default_page_id, PageId};

#[derive(Debug, Clone, Deserialize)]
pub struct AdvancedSearchQuery {
    #[serde(default = "default_page_id")]
    pub page: PageId,
    #[serde(default = "GetBooksOrder::default")]
    pub order: GetBooksOrder,

    pub title: Option<String>,
    pub author: Option<String>,
    pub publisher: Option<String>,
}

pub fn get_books_by_advanced_search(
    conn: &mut PgConnection,
    query: &AdvancedSearchQuery,
) -> Result<GetBooksResp, Error> {
    use crate::schema::books;

    let books_query = GetBooksQuery {
        page: query.page,
        order: query.order,
    };

    // TODO(Shaohua): Join query
    let empty_title = "".to_owned();
    let book_title = query.title.as_ref().unwrap_or(&empty_title);
    let title_pattern = format!("%{}%", book_title);

    let book_ids = books::table
        .filter(books::title.ilike(&title_pattern))
        .select(books::id)
        .load::<i32>(conn)?;

    get_books_by_ids(conn, &books_query, &book_ids)
}
