// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use diesel::{ExpressionMethods, Insertable, JoinOnDsl, PgConnection, QueryDsl, RunQueryDsl};
use serde::Deserialize;
use shared::page::{Page, USER_TAGS_EACH_PAGE};
use shared::recursive_query::RecursiveQuery;
use shared::user_tags::{UserTag, UserTagAndBook, UserTagAndBookList};

use crate::error::Error;
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

pub fn get_tags(
    conn: &mut PgConnection,
    query: &RecursiveQuery,
) -> Result<UserTagAndBookList, Error> {
    use crate::schema::books_user_tags_link;

    let page_id = if query.page < 1 { 0 } else { query.page - 1 };
    let offset = page_id * USER_TAGS_EACH_PAGE;

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
        .limit(USER_TAGS_EACH_PAGE)
        .offset(offset)
        .load::<UserTagAndBook>(conn)?;

    let total = user_tags::table
        .filter(user_tags::parent.eq(query.parent))
        .count()
        .first(conn)?;

    Ok(UserTagAndBookList {
        page: Page {
            page_num: page_id + 1,
            each_page: USER_TAGS_EACH_PAGE,
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
