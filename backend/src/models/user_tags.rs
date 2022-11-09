// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use diesel::{ExpressionMethods, Insertable, JoinOnDsl, PgConnection, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};
use shared::books::BookAndAuthorsList;
use shared::books_query::GetBooksQuery;
use shared::page::{default_page_id, Page};
use shared::user_tags::{UserTag, UserTagAndBook, UserTagAndBookList};

use crate::error::Error;
use crate::models::books::get_books_by_ids;
use crate::schema::user_tags;

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = user_tags)]
pub struct NewUserTag {
    pub name: String,
}

pub fn add_tag(conn: &mut PgConnection, new_tag: &NewUserTag) -> Result<(), Error> {
    diesel::insert_into(user_tags::table)
        .values(new_tag)
        .execute(conn)?;
    Ok(())
}

pub fn get_tag_by_id(conn: &mut PgConnection, tag_id: i32) -> Result<UserTag, Error> {
    user_tags::table
        .find(tag_id)
        .first::<UserTag>(conn)
        .map_err(Into::into)
}

pub fn get_tag_by_name(conn: &mut PgConnection, tag_name: &str) -> Result<UserTag, Error> {
    user_tags::table
        .filter(user_tags::name.eq(tag_name))
        .first::<UserTag>(conn)
        .map_err(Into::into)
}

#[must_use]
pub const fn default_parent_tag_id() -> i32 {
    0
}

// TODO(Shaohua): Replace with query

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GetUserTagsReq {
    #[serde(default = "default_parent_tag_id")]
    pub parent: i32,
    #[serde(default = "default_page_id")]
    pub page: i64,
}

pub fn get_tags(
    conn: &mut PgConnection,
    query: &GetUserTagsReq,
) -> Result<UserTagAndBookList, Error> {
    use crate::schema::books_user_tags_link;

    let page_id = if query.page < 1 { 0 } else { query.page - 1 };
    let each_page = 100;
    let offset = page_id * each_page;

    let list = user_tags::table
        .filter(user_tags::parent.eq(query.parent))
        .left_join(books_user_tags_link::table.on(books_user_tags_link::tag.eq(user_tags::id)))
        .group_by(user_tags::id)
        .select((
            user_tags::id,
            user_tags::order_index,
            user_tags::name,
            user_tags::parent,
            diesel::dsl::sql::<diesel::sql_types::BigInt>("count(books_user_tags_link.id)"),
        ))
        .limit(each_page)
        .offset(offset)
        .load::<UserTagAndBook>(conn)?;

    let total = user_tags::table
        .filter(user_tags::parent.eq(query.parent))
        .count()
        .first(conn)?;

    Ok(UserTagAndBookList {
        page: Page {
            page_num: page_id + 1,
            each_page,
            total,
        },
        list,
    })
}

pub fn update_tag(conn: &mut PgConnection, tag_id: i32, new_tag: &NewUserTag) -> Result<(), Error> {
    diesel::update(user_tags::table.find(tag_id))
        .set(user_tags::name.eq(new_tag.name.as_str()))
        .execute(conn)?;
    Ok(())
}

pub fn get_books_by_user_tag(
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
