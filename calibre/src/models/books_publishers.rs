// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SqliteConnection};

use crate::error::Error;

#[derive(Debug, Queryable)]
pub struct BookPublisher {
    pub book: i32,
    pub name: String,
}

pub fn get_book_publisher(conn: &SqliteConnection, book_id: i32) -> Result<BookPublisher, Error> {
    use crate::schema::{books_publishers_link, publishers};

    books_publishers_link::dsl::books_publishers_link
        .filter(books_publishers_link::book.eq(book_id))
        .inner_join(publishers::table.on(books_publishers_link::publisher.eq(publishers::id)))
        .select((books_publishers_link::book, publishers::name))
        .first::<BookPublisher>(conn)
        .map_err(Into::into)
}
