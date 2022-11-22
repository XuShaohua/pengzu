// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use shared::books::BookAndAuthorsList;
use shared::books_query::GetBooksQuery;

use crate::error::Error;
use crate::models::books::get_books_by_ids;

pub fn get_books(
    conn: &mut PgConnection,
    tag_id: i32,
    query: &GetBooksQuery,
) -> Result<BookAndAuthorsList, Error> {
    use crate::schema::books_user_tags_link;

    let book_ids = books_user_tags_link::table
        .filter(books_user_tags_link::tag.eq(tag_id))
        .select(books_user_tags_link::book)
        .load::<i32>(conn)?;

    get_books_by_ids(conn, query, &book_ids)
}
