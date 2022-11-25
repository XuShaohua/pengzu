// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use diesel::{PgConnection, PgTextExpressionMethods, QueryDsl, RunQueryDsl};
use shared::books::BookAndAuthorsList;
use shared::books_query::GetBooksQuery;
use shared::page::BOOKS_EACH_PAGE;
use shared::simple_search::SimpleSearchQuery;

use crate::error::Error;
use crate::models::books::{book_list_to_book_authors, Book};
use crate::models::books_query::sort_books_by_column;

pub fn get_books_by_simple_search(
    conn: &mut PgConnection,
    query: &SimpleSearchQuery,
) -> Result<BookAndAuthorsList, Error> {
    use crate::schema::books;

    let books_query = GetBooksQuery {
        page: query.page,
        order: query.order,
    };
    let offset = books_query.backend_page_id() * BOOKS_EACH_PAGE;
    let query_pattern = format!("%{}%", query.query);

    let total = books::table
        .filter(books::title.ilike(&query_pattern))
        .count()
        .first::<i64>(conn)?;

    // Get book list based on a subquery.
    let book_list = sort_books_by_column(query.order)
        .filter(books::title.ilike(&query_pattern))
        .limit(BOOKS_EACH_PAGE)
        .offset(offset)
        .load::<Book>(conn)?;

    book_list_to_book_authors(conn, book_list, &books_query, total)
}
