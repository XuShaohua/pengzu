// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use shared::books::BookAndAuthorsList;
use shared::books_query::GetBooksQuery;
use shared::page::BOOKS_EACH_PAGE;

use crate::error::Error;
use crate::models::books::{book_list_to_book_authors, Book};
use crate::models::books_query::sort_books_by_column;
use crate::schema::books_categories_link;

pub fn get_books_by_category(
    conn: &mut PgConnection,
    category_id: i32,
    query: &GetBooksQuery,
) -> Result<BookAndAuthorsList, Error> {
    use crate::schema::books;

    let offset = query.backend_page_id() * BOOKS_EACH_PAGE;
    let total = books_categories_link::table
        .filter(books_categories_link::category.eq(category_id))
        .count()
        .first::<i64>(conn)?;

    // Get book list based on a subquery.
    let book_list = sort_books_by_column(query.order)
        .filter(
            books::id.eq_any(
                books_categories_link::table
                    .filter(books_categories_link::category.eq(category_id))
                    .select(books_categories_link::book),
            ),
        )
        .limit(BOOKS_EACH_PAGE)
        .offset(offset)
        .load::<Book>(conn)?;

    book_list_to_book_authors(conn, book_list, query, total)
}
