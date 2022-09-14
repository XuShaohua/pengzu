// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SqliteConnection};

use crate::error::Error;

#[derive(Debug, Queryable)]
pub struct BookRating {
    pub book: i32,
    pub rating: i32,
}

pub fn get_book_rating(conn: &mut SqliteConnection, book_id: i32) -> Result<BookRating, Error> {
    use crate::schema::{books_ratings_link, ratings};

    books_ratings_link::dsl::books_ratings_link
        .filter(books_ratings_link::book.eq(book_id))
        .inner_join(ratings::table.on(books_ratings_link::rating.eq(ratings::id)))
        .select((books_ratings_link::book, ratings::dsl::rating))
        .first::<BookRating>(conn)
        .map_err(Into::into)
}
