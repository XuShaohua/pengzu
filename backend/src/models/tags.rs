// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use diesel::{
    ExpressionMethods, Insertable, JoinOnDsl, PgConnection, PgTextExpressionMethods, QueryDsl,
    RunQueryDsl,
};
use serde::Deserialize;
use shared::general_query::GeneralOrder;
use shared::page::{Page, TAGS_EACH_PAGE};
use shared::recursive_query::RecursiveQuery;
use shared::tags::{Tag, TagAndBook, TagAndBookList};

use crate::error::Error;
use crate::schema::tags;

#[derive(Debug, Default, Deserialize, Insertable)]
#[diesel(table_name = tags)]
pub struct NewTag {
    pub order_index: i32,
    pub name: String,
    pub parent: i32,
}

impl NewTag {
    #[must_use]
    pub const fn with_name(name: String) -> Self {
        Self {
            order_index: 0,
            name,
            parent: 0,
        }
    }
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

pub fn get_tags(conn: &mut PgConnection, query: &RecursiveQuery) -> Result<TagAndBookList, Error> {
    use crate::schema::books_tags_link;

    let offset = query.backend_page_id() * TAGS_EACH_PAGE;

    // TODO(Shaohua): Get children count.
    let count_query = diesel::dsl::sql::<diesel::sql_types::BigInt>("count(books_tags_link.id)");
    let child_count_query =
        diesel::dsl::sql::<diesel::sql_types::BigInt>("count(books_tags_link.id)");

    let stmt = tags::table
        .left_join(books_tags_link::table.on(books_tags_link::tag.eq(tags::id)))
        .group_by(tags::id)
        .select((
            tags::id,
            tags::order_index,
            tags::name,
            tags::parent,
            count_query.clone(),
            child_count_query,
        ))
        .limit(TAGS_EACH_PAGE)
        .offset(offset);

    let list = match query.order {
        GeneralOrder::IdDesc => stmt.order(tags::id.desc()).load::<TagAndBook>(conn),
        GeneralOrder::IdAsc => stmt.order(tags::id.asc()).load::<TagAndBook>(conn),
        GeneralOrder::TitleDesc => stmt.order(tags::name.desc()).load::<TagAndBook>(conn),
        GeneralOrder::TitleAsc => stmt.order(tags::name.asc()).load::<TagAndBook>(conn),
        GeneralOrder::NumberDesc => stmt.order(count_query.desc()).load::<TagAndBook>(conn),
        GeneralOrder::NumberAsc => stmt.order(count_query.asc()).load::<TagAndBook>(conn),
    }?;

    let total = tags::table
        .filter(tags::parent.eq(query.parent))
        .count()
        .first(conn)?;

    Ok(TagAndBookList {
        page: Page {
            page_num: query.frontend_page_id(),
            each_page: TAGS_EACH_PAGE,
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

pub fn delete_by_id(conn: &mut PgConnection, tag_id: i32) -> Result<(), Error> {
    diesel::delete(tags::table.find(tag_id)).execute(conn)?;
    Ok(())
}
