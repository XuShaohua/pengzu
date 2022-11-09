// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use diesel::{PgConnection, PgTextExpressionMethods, QueryDsl, RunQueryDsl};
use shared::books::BookAndAuthorsList;
use shared::books_query::GetBooksQuery;
use shared::simple_search::SimpleSearchQuery;

use crate::error::Error;
use crate::models::books::get_books_by_ids;

pub fn get_books_by_simple_search(
    conn: &mut PgConnection,
    query: &SimpleSearchQuery,
) -> Result<BookAndAuthorsList, Error> {
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
