// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use shared::books::BookAndAuthorsList;
use shared::books_query::GetBooksQuery;
use shared::page::BOOKS_EACH_PAGE;
use shared::user_tags::UserTag;

use crate::error::Error;
use crate::models::books::{book_list_to_book_authors, Book};
use crate::models::books_query::sort_books_by_column;
use crate::schema::books_user_tags_link;

pub fn get_books_by_user_tag(
    conn: &mut PgConnection,
    tag_id: i32,
    query: &GetBooksQuery,
) -> Result<BookAndAuthorsList, Error> {
    use crate::schema::books;

    let offset = query.backend_page_id() * BOOKS_EACH_PAGE;
    let total = books_user_tags_link::table
        .filter(books_user_tags_link::tag.eq(tag_id))
        .count()
        .first::<i64>(conn)?;

    // Get book list based on a subquery.
    let book_list = sort_books_by_column(query.order)
        .filter(
            books::id.eq_any(
                books_user_tags_link::table
                    .filter(books_user_tags_link::tag.eq(tag_id))
                    .select(books_user_tags_link::book),
            ),
        )
        .limit(BOOKS_EACH_PAGE)
        .offset(offset)
        .load::<Book>(conn)?;

    book_list_to_book_authors(conn, book_list, query, total)
}

pub fn get_user_tags_by_book(
    conn: &mut PgConnection,
    user_id: i32,
    book_id: i32,
) -> Result<Vec<UserTag>, Error> {
    use crate::schema::user_tags;

    // Replace INNER JOIN with a subquery.
    user_tags::table
        .filter(
            user_tags::id.eq_any(
                books_user_tags_link::table
                    .filter(books_user_tags_link::book.eq(book_id))
                    .filter(books_user_tags_link::user_id.eq(user_id))
                    .select(books_user_tags_link::tag),
            ),
        )
        .load::<UserTag>(conn)
        .map_err(Into::into)
}

pub fn delete_by_tag_id(conn: &mut PgConnection, tag_id: i32, user_id: i32) -> Result<(), Error> {
    diesel::delete(
        books_user_tags_link::table
            .filter(books_user_tags_link::tag.eq(tag_id))
            .filter(books_user_tags_link::user_id.eq(user_id)),
    )
    .execute(conn)?;
    Ok(())
}
