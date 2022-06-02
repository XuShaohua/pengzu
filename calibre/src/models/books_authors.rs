// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SqliteConnection};

use crate::error::Error;

#[derive(Debug, Queryable)]
pub struct BookAuthor {
    pub book: i32,
    pub name: String,
}

pub fn get_book_authors(conn: &SqliteConnection, book_id: i32) -> Result<Vec<BookAuthor>, Error> {
    use crate::schema::authors;
    use crate::schema::books_authors_link;

    books_authors_link::dsl::books_authors_link
        .filter(books_authors_link::book.eq(book_id))
        .inner_join(authors::table.on(books_authors_link::author.eq(authors::id)))
        .select((books_authors_link::book, authors::name))
        .load::<BookAuthor>(conn)
        .map_err(Into::into)
}
