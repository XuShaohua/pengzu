// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use chrono::NaiveDateTime;
use diesel::{
    ExpressionMethods, Insertable, JoinOnDsl, PgConnection, QueryDsl, Queryable, RunQueryDsl,
};
use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::models::tags::Tag;
use crate::schema::books_tags_link;

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = books_tags_link)]
pub struct NewBookTag {
    pub book: i32,
    pub tag: i32,
}

#[derive(Debug, Serialize, Queryable)]
pub struct BookTag {
    pub id: i32,
    pub book: i32,
    pub tag: i32,
    pub created: NaiveDateTime,
}

pub fn add_book_tag(conn: &mut PgConnection, new_book_tag: &NewBookTag) -> Result<(), Error> {
    use crate::schema::books_tags_link::dsl::books_tags_link;
    diesel::insert_into(books_tags_link)
        .values(new_book_tag)
        .execute(conn)?;
    Ok(())
}

pub fn get_tags_by_book(conn: &mut PgConnection, book_id: i32) -> Result<Vec<Tag>, Error> {
    use crate::schema::tags;
    tags::table
        .inner_join(books_tags_link::table.on(books_tags_link::tag.eq(tags::id)))
        .filter(books_tags_link::book.eq(book_id))
        .select((
            tags::id,
            tags::order_index,
            tags::name,
            tags::parent,
            tags::created,
            tags::last_modified,
        ))
        .load::<Tag>(conn)
        .map_err(Into::into)
}

pub fn get_links_by_tag(conn: &mut PgConnection, tag_id: i32) -> Result<Vec<BookTag>, Error> {
    books_tags_link::table
        .filter(books_tags_link::tag.eq(tag_id))
        .load::<BookTag>(conn)
        .map_err(Into::into)
}

pub fn delete_book_tag(conn: &mut PgConnection, new_book_tag: &NewBookTag) -> Result<(), Error> {
    use crate::schema::books_tags_link::dsl::{book, books_tags_link, tag};
    // TODO(Shaohua): Check exists
    diesel::delete(
        books_tags_link
            .filter(book.eq(new_book_tag.book))
            .filter(tag.eq(new_book_tag.tag)),
    )
    .execute(conn)?;
    Ok(())
}

pub fn delete_by_id(conn: &mut PgConnection, id: i32) -> Result<(), Error> {
    diesel::delete(books_tags_link::table.find(id)).execute(conn)?;
    Ok(())
}
