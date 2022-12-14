// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};
use shared::books::BookAndAuthorsList;
use shared::books_query::GetBooksQuery;
use shared::page::BOOKS_EACH_PAGE;
use shared::tags::Tag;

use crate::error::Error;
use crate::models::books::{book_list_to_book_authors, Book};
use crate::models::books_query::sort_books_by_column;
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
    log::info!("book id: {}", book_id);

    // Replace INNER JOIN with a subquery.
    tags::table
        .filter(
            tags::id.eq_any(
                books_tags_link::table
                    .filter(books_tags_link::book.eq(book_id))
                    .select(books_tags_link::tag),
            ),
        )
        .load::<Tag>(conn)
        .map_err(Into::into)
}

pub fn get_links_by_tag(conn: &mut PgConnection, tag_id: i32) -> Result<Vec<BookTag>, Error> {
    books_tags_link::table
        .filter(books_tags_link::tag.eq(tag_id))
        .load::<BookTag>(conn)
        .map_err(Into::into)
}

pub fn delete_by_id(conn: &mut PgConnection, id: i32) -> Result<(), Error> {
    diesel::delete(books_tags_link::table.find(id)).execute(conn)?;
    Ok(())
}

pub fn delete_by_tag_id(conn: &mut PgConnection, tag_id: i32) -> Result<(), Error> {
    diesel::delete(books_tags_link::table.filter(books_tags_link::tag.eq(tag_id))).execute(conn)?;
    Ok(())
}

pub fn get_books_by_tag(
    conn: &mut PgConnection,
    tag_id: i32,
    query: &GetBooksQuery,
) -> Result<BookAndAuthorsList, Error> {
    use crate::schema::books;

    let offset = query.backend_page_id() * BOOKS_EACH_PAGE;
    let total = books_tags_link::table
        .filter(books_tags_link::tag.eq(tag_id))
        .count()
        .first::<i64>(conn)?;

    // Get book list based on a subquery.
    let book_list = sort_books_by_column(query.order)
        .filter(
            books::id.eq_any(
                books_tags_link::table
                    .filter(books_tags_link::tag.eq(tag_id))
                    .select(books_tags_link::book),
            ),
        )
        .limit(BOOKS_EACH_PAGE)
        .offset(offset)
        .load::<Book>(conn)?;

    book_list_to_book_authors(conn, book_list, query, total)
}
