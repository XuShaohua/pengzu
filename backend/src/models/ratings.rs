// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::schema::ratings;

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "ratings"]
pub struct NewRating {
    pub book: i32,
    pub rating: i32,
}

#[derive(Debug, Serialize, Queryable)]
pub struct Rating {
    pub id: i32,
    pub book: i32,
    pub rating: i32,
    pub created: NaiveDateTime,
    pub last_modified: NaiveDateTime,
}

pub fn add_rating(conn: &PgConnection, new_rating: &NewRating) -> Result<(), Error> {
    use crate::schema::ratings::dsl::ratings;
    diesel::insert_into(ratings)
        .values(new_rating)
        .execute(conn)?;
    Ok(())
}

pub fn get_rating(conn: &PgConnection, book_id: i32) -> Result<Rating, Error> {
    use crate::schema::ratings::dsl::{book, ratings};
    ratings
        .filter(book.eq(book_id))
        .first::<Rating>(conn)
        .map_err(Into::into)
}

pub fn update_rating(conn: &PgConnection, new_rating: &NewRating) -> Result<(), Error> {
    use crate::schema::ratings::dsl::{book, rating, ratings};
    diesel::update(ratings.filter(book.eq(new_rating.book)))
        .set(rating.eq(new_rating.rating))
        .execute(conn)?;
    Ok(())
}

pub fn delete_rating(conn: &PgConnection, book_id: i32) -> Result<(), Error> {
    use crate::schema::ratings::dsl::{book, ratings};
    let _rating = get_rating(conn, book_id)?;
    diesel::delete(ratings.filter(book.eq(book_id))).execute(conn)?;
    Ok(())
}
