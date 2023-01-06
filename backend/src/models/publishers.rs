// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use diesel::{ExpressionMethods, Insertable, JoinOnDsl, PgConnection, QueryDsl, RunQueryDsl};
use serde::Deserialize;
use shared::general_query::{GeneralOrder, GeneralQuery};
use shared::page::{Page, PUBLISHERS_EACH_PAGE};
use shared::publishers::{Publisher, PublisherAndBook, PublisherAndBookList};

use crate::error::Error;
use crate::schema::publishers;

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = publishers)]
pub struct NewPublisher {
    pub name: String,
}

pub fn add_publisher(conn: &mut PgConnection, new_publisher: &NewPublisher) -> Result<(), Error> {
    use crate::schema::publishers::dsl::publishers;
    diesel::insert_into(publishers)
        .values(new_publisher)
        .execute(conn)?;
    Ok(())
}

pub fn get_publishers(
    conn: &mut PgConnection,
    query: &GeneralQuery,
) -> Result<PublisherAndBookList, Error> {
    use crate::schema::books_publishers_link;

    let offset = query.backend_page_id() * PUBLISHERS_EACH_PAGE;

    let count_query =
        diesel::dsl::sql::<diesel::sql_types::BigInt>("count(books_publishers_link.id)");
    let stmt = publishers::table
        .left_join(
            books_publishers_link::table.on(books_publishers_link::publisher.eq(publishers::id)),
        )
        .group_by(publishers::id)
        .select((publishers::id, publishers::name, count_query.clone()))
        .limit(PUBLISHERS_EACH_PAGE)
        .offset(offset);

    let list = match query.order {
        GeneralOrder::IdDesc => stmt
            .order(publishers::id.desc())
            .load::<PublisherAndBook>(conn),
        GeneralOrder::IdAsc => stmt
            .order(publishers::id.asc())
            .load::<PublisherAndBook>(conn),
        GeneralOrder::TitleDesc => stmt
            .order(publishers::name.desc())
            .load::<PublisherAndBook>(conn),
        GeneralOrder::TitleAsc => stmt
            .order(publishers::name.asc())
            .load::<PublisherAndBook>(conn),
        GeneralOrder::NumberDesc => stmt
            .order(count_query.desc())
            .load::<PublisherAndBook>(conn),
        GeneralOrder::NumberAsc => stmt.order(count_query.asc()).load::<PublisherAndBook>(conn),
    }?;

    let total = publishers::dsl::publishers.count().first(conn)?;

    Ok(PublisherAndBookList {
        page: Page {
            page_num: query.frontend_page_id(),
            each_page: PUBLISHERS_EACH_PAGE,
            total,
        },
        list,
    })
}

pub fn get_publisher_by_id(conn: &mut PgConnection, publisher_id: i32) -> Result<Publisher, Error> {
    publishers::table
        .find(publisher_id)
        .first(conn)
        .map_err(Into::into)
}

pub fn get_publisher_by_name(
    conn: &mut PgConnection,
    publisher_name: &str,
) -> Result<Publisher, Error> {
    use crate::schema::publishers::dsl::{name, publishers};
    publishers
        .filter(name.eq(publisher_name))
        .first(conn)
        .map_err(Into::into)
}

pub fn update_publisher(
    conn: &mut PgConnection,
    pub_id: i32,
    new_publisher: &NewPublisher,
) -> Result<(), Error> {
    use crate::schema::publishers::dsl::{name, publishers};
    diesel::update(publishers.find(pub_id))
        .set(name.eq(new_publisher.name.as_str()))
        .execute(conn)?;
    Ok(())
}

pub fn delete_by_id(conn: &mut PgConnection, publisher_id: i32) -> Result<(), Error> {
    diesel::delete(publishers::table.find(publisher_id)).execute(conn)?;
    Ok(())
}
