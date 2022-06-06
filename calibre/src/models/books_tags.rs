// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SqliteConnection};

use crate::error::Error;

#[derive(Debug, Queryable)]
pub struct BookTag {
    pub book: i32,
    pub name: String,
}

pub fn get_book_tags(conn: &SqliteConnection, book_id: i32) -> Result<Vec<BookTag>, Error> {
    use crate::schema::{books_tags_link, tags};

    books_tags_link::dsl::books_tags_link
        .filter(books_tags_link::book.eq(book_id))
        .inner_join(tags::table.on(books_tags_link::tag.eq(tags::id)))
        .select((books_tags_link::book, tags::name))
        .load::<BookTag>(conn)
        .map_err(Into::into)
}
