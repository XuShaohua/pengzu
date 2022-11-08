// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use chrono::NaiveDateTime;
use diesel::{
    ExpressionMethods, Insertable, JoinOnDsl, PgConnection, PgTextExpressionMethods, QueryDsl,
    Queryable, RunQueryDsl,
};
use serde::{Deserialize, Serialize};

use super::page::{default_page_id, Page};
use crate::error::Error;
use crate::models::books::{get_books_by_ids, GetBooksQuery, GetBooksResp};
use crate::schema::tags;

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = tags)]
pub struct NewTag {
    pub name: String,
}

#[derive(Debug, Serialize, Queryable)]
pub struct Tag {
    pub id: i32,
    pub order_index: i32,
    pub name: String,
    pub parent: i32,
    pub created: NaiveDateTime,
    pub last_modified: NaiveDateTime,
}

pub fn add_tag(conn: &mut PgConnection, new_tag: &NewTag) -> Result<Tag, Error> {
    use crate::schema::tags::dsl::tags;
    diesel::insert_into(tags)
        .values(new_tag)
        .get_result::<Tag>(conn)
        .map_err(Into::into)
}

pub fn get_tag_by_id(conn: &mut PgConnection, tag_id: i32) -> Result<Tag, Error> {
    tags::table.find(tag_id).first(conn).map_err(Into::into)
}

pub fn get_tag_by_name(conn: &mut PgConnection, tag_name: &str) -> Result<Tag, Error> {
    use crate::schema::tags::dsl::{name, tags};
    tags.filter(name.eq(tag_name))
        .first(conn)
        .map_err(Into::into)
}

pub fn get_tag_by_name_pattern(conn: &mut PgConnection, name_pattern: &str) -> Result<Tag, Error> {
    tags::table
        .filter(tags::name.ilike(name_pattern))
        .first(conn)
        .map_err(Into::into)
}

#[derive(Debug, Serialize, Queryable)]
pub struct TagAndBook {
    pub id: i32,
    pub order_index: i32,
    pub name: String,
    pub parent: i32,
    pub count: i64,
}

#[derive(Debug, Serialize)]
pub struct GetTagsResp {
    pub page: Page,
    pub list: Vec<TagAndBook>,
}

#[must_use]
pub const fn default_parent_tag_id() -> i32 {
    0
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GetTagsReq {
    #[serde(default = "default_parent_tag_id")]
    pub parent: i32,
    #[serde(default = "default_page_id")]
    pub page: i64,
}

pub fn get_tags(conn: &mut PgConnection, query: &GetTagsReq) -> Result<GetTagsResp, Error> {
    use crate::schema::books_tags_link;

    let page_id = if query.page < 1 { 0 } else { query.page - 1 };
    let each_page = 100;
    let offset = page_id * each_page;

    let list = tags::table
        .filter(tags::parent.eq(query.parent))
        .left_join(books_tags_link::table.on(books_tags_link::tag.eq(tags::id)))
        .group_by(tags::id)
        .select((
            tags::id,
            tags::order_index,
            tags::name,
            tags::parent,
            diesel::dsl::sql::<diesel::sql_types::BigInt>("count(books_tags_link.id)"),
        ))
        .limit(each_page)
        .offset(offset)
        .load::<TagAndBook>(conn)?;

    let total = tags::table
        .filter(tags::parent.eq(query.parent))
        .count()
        .first(conn)?;

    Ok(GetTagsResp {
        page: Page {
            page_num: page_id + 1,
            each_page,
            total,
        },
        list,
    })
}

pub fn update_tag(conn: &mut PgConnection, tag_id: i32, new_tag: &NewTag) -> Result<(), Error> {
    use crate::schema::tags::dsl::{name, tags};
    diesel::update(tags.find(tag_id))
        .set(name.eq(new_tag.name.as_str()))
        .execute(conn)?;
    Ok(())
}

pub fn get_books_by_tag(
    conn: &mut PgConnection,
    tag_id: i32,
    query: &GetBooksQuery,
) -> Result<GetBooksResp, Error> {
    use crate::schema::books_tags_link;

    let book_ids = books_tags_link::table
        .filter(books_tags_link::tag.eq(tag_id))
        .select(books_tags_link::book)
        .load::<i32>(conn)?;

    get_books_by_ids(conn, query, &book_ids)
}

pub fn delete_by_id(conn: &mut PgConnection, id: i32) -> Result<(), Error> {
    diesel::delete(tags::table.find(id)).execute(conn)?;
    Ok(())
}
