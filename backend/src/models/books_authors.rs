// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use diesel::{ExpressionMethods, Insertable, JoinOnDsl, PgConnection, QueryDsl, RunQueryDsl};
use serde::Deserialize;

use crate::error::Error;
use crate::models::authors::Author;
use crate::schema::books_authors_link;

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "books_authors_link"]
pub struct NewBookAuthor {
    pub book: i32,
    pub author: i32,
}

pub fn add_book_author(
    conn: &mut PgConnection,
    new_book_author: &NewBookAuthor,
) -> Result<(), Error> {
    use crate::schema::books_authors_link::dsl::books_authors_link;
    diesel::insert_into(books_authors_link)
        .values(new_book_author)
        .execute(conn)?;
    Ok(())
}

pub fn delete_book_author(
    conn: &mut PgConnection,
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

pub fn get_authors_by_book(conn: &mut PgConnection, book_id: i32) -> Result<Vec<Author>, Error> {
    use crate::schema::authors;

    authors::table
        .inner_join(books_authors_link::table.on(books_authors_link::author.eq(authors::id)))
        .filter(books_authors_link::book.eq(book_id))
        .select((
            authors::id,
            authors::name,
            authors::link,
            authors::created,
            authors::last_modified,
        ))
        .load::<Author>(conn)
        .map_err(Into::into)
}
