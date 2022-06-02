// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use diesel::{ExpressionMethods, Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::schema::books_authors_link;

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "books_authors_link"]
pub struct NewBookAuthor {
    pub book: i32,
    pub author: i32,
}

#[derive(Debug, Serialize, Queryable)]
pub struct BookAuthor {
    pub id: i32,
    pub book: i32,
    pub author: i32,
}

pub fn add_book_author(conn: &PgConnection, new_book_author: &NewBookAuthor) -> Result<(), Error> {
    use crate::schema::books_authors_link::dsl::books_authors_link;
    diesel::insert_into(books_authors_link)
        .values(new_book_author)
        .execute(conn)?;
    Ok(())
}

pub fn get_book_author(conn: &PgConnection, book_id: i32) -> Result<Vec<BookAuthor>, Error> {
    use crate::schema::books_authors_link::dsl::{book, books_authors_link};
    books_authors_link
        .filter(book.eq(book_id))
        .load::<BookAuthor>(conn)
        .map_err(Into::into)
}

pub fn delete_book_author(
    conn: &PgConnection,
    new_book_author: &NewBookAuthor,
) -> Result<(), Error> {
    use crate::schema::books_authors_link::dsl::{author, book, books_authors_link};
    // TODO(Shaohua): Check exists
    diesel::delete(
        books_authors_link
            .filter(book.eq(new_book_author.book))
            .filter(author.eq(new_book_author.author)),
    )
    .execute(conn)?;
    Ok(())
}
