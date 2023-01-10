// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use diesel::{ExpressionMethods, Insertable, JoinOnDsl, PgConnection, QueryDsl, RunQueryDsl};
use serde::Deserialize;
use shared::general_query::GeneralOrder;
use shared::page::{Page, USER_TAGS_EACH_PAGE};
use shared::recursive_query::RecursiveQuery;
use shared::user_tags::{UserTag, UserTagAndBook, UserTagAndBookList};

use crate::error::Error;
use crate::schema::user_tags;

#[derive(Debug, Clone, Deserialize, Insertable)]
#[diesel(table_name = user_tags)]
pub struct NewUserTag {
    pub user_id: i32,
    pub order_index: i32,
    pub name: String,
    pub parent: i32,
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

    let offset = query.backend_page_id() * USER_TAGS_EACH_PAGE;

    // TODO(Shaohua): Get child count.
    let count_query =
        diesel::dsl::sql::<diesel::sql_types::BigInt>("count(books_user_tags_link.id)");
    let stmt = user_tags::table
        .left_join(books_user_tags_link::table.on(books_user_tags_link::tag.eq(user_tags::id)))
        .group_by(user_tags::id)
        .select((
            user_tags::id,
            user_tags::order_index,
            user_tags::name,
            user_tags::parent,
            count_query.clone(),
        ))
        .filter(user_tags::parent.eq(query.parent))
        .limit(USER_TAGS_EACH_PAGE)
        .offset(offset);

    let list = match query.order {
        GeneralOrder::IdDesc => stmt
            .order(user_tags::id.desc())
            .load::<UserTagAndBook>(conn),
        GeneralOrder::IdAsc => stmt.order(user_tags::id.asc()).load::<UserTagAndBook>(conn),
        GeneralOrder::TitleDesc => stmt
            .order(user_tags::name.desc())
            .load::<UserTagAndBook>(conn),
        GeneralOrder::TitleAsc => stmt
            .order(user_tags::name.asc())
            .load::<UserTagAndBook>(conn),
        GeneralOrder::NumberDesc => stmt.order(count_query.desc()).load::<UserTagAndBook>(conn),
        GeneralOrder::NumberAsc => stmt.order(count_query.asc()).load::<UserTagAndBook>(conn),
    }?;

    let total = user_tags::table
        .filter(user_tags::parent.eq(query.parent))
        .count()
        .first(conn)?;

    Ok(UserTagAndBookList {
        page: Page {
            page_num: query.frontend_page_id(),
            each_page: USER_TAGS_EACH_PAGE,
            total,
        },
        list,
    })
}

pub fn update_tag(conn: &mut PgConnection, tag_id: i32, new_tag: &NewUserTag) -> Result<(), Error> {
    diesel::update(
        user_tags::table
            .filter(user_tags::id.eq(tag_id))
            .filter(user_tags::user_id.eq(new_tag.user_id)),
    )
    .set(user_tags::name.eq(new_tag.name.as_str()))
    .execute(conn)?;
    Ok(())
}

pub fn delete_by_id(conn: &mut PgConnection, tag_id: i32, user_id: i32) -> Result<(), Error> {
    diesel::delete(
        user_tags::table
            .filter(user_tags::id.eq(tag_id))
            .filter(user_tags::user_id.eq(user_id)),
    )
    .execute(conn)?;
    Ok(())
}
