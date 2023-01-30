// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use diesel::{ExpressionMethods, Insertable, JoinOnDsl, PgConnection, QueryDsl, RunQueryDsl};
use serde::Deserialize;
use shared::general_query::{GeneralOrder, GeneralQuery};
use shared::page::{Page, SERIES_EACH_PAGE};
use shared::series::{Series, SeriesAndBook, SeriesAndBookList};

use crate::error::Error;
use crate::schema::series;

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = series)]
pub struct NewSeries {
    pub name: String,
}

pub fn add_series(conn: &mut PgConnection, new_series: &NewSeries) -> Result<(), Error> {
    use crate::schema::series::dsl::series;
    diesel::insert_into(series)
        .values(new_series)
        .execute(conn)?;
    Ok(())
}

pub fn get_series_list(
    conn: &mut PgConnection,
    query: &GeneralQuery,
) -> Result<SeriesAndBookList, Error> {
    use crate::schema::books_series_link;

    let offset = query.backend_page_id() * SERIES_EACH_PAGE;

    let count_query = diesel::dsl::sql::<diesel::sql_types::BigInt>("count(books_series_link.id)");
    let stmt = series::table
        .left_join(books_series_link::table.on(books_series_link::series.eq(series::id)))
        .group_by(series::id)
        .select((series::id, series::name, count_query.clone()))
        .limit(SERIES_EACH_PAGE)
        .offset(offset);
    let list = match query.order {
        GeneralOrder::IdDesc => stmt.order(series::id.desc()).load::<SeriesAndBook>(conn),
        GeneralOrder::IdAsc => stmt.order(series::id.asc()).load::<SeriesAndBook>(conn),
        GeneralOrder::TitleDesc => stmt
            .order(series::name.desc())
            .then_order_by(series::id.desc())
            .load::<SeriesAndBook>(conn),
        GeneralOrder::TitleAsc => stmt
            .order(series::name.asc())
            .then_order_by(series::id.desc())
            .load::<SeriesAndBook>(conn),
        GeneralOrder::NumberDesc => stmt
            .order(count_query.desc())
            .then_order_by(series::id.desc())
            .load::<SeriesAndBook>(conn),
        GeneralOrder::NumberAsc => stmt
            .order(count_query.asc())
            .then_order_by(series::id.desc())
            .load::<SeriesAndBook>(conn),
    }?;

    let total = series::dsl::series.count().first(conn)?;

    Ok(SeriesAndBookList {
        page: Page {
            page_num: query.frontend_page_id(),
            each_page: SERIES_EACH_PAGE,
            total,
        },
        list,
    })
}

pub fn get_series_by_id(conn: &mut PgConnection, series_id: i32) -> Result<Series, Error> {
    series::table
        .find(series_id)
        .first(conn)
        .map_err(Into::into)
}

pub fn get_series_by_name(conn: &mut PgConnection, series_name: &str) -> Result<Series, Error> {
    use crate::schema::series::dsl::{name, series};
    series
        .filter(name.eq(series_name))
        .first(conn)
        .map_err(Into::into)
}

pub fn update_series(
    conn: &mut PgConnection,
    series_id: i32,
    new_series: &NewSeries,
) -> Result<(), Error> {
    use crate::schema::series::dsl::{name, series};
    diesel::update(series.find(series_id))
        .set(name.eq(new_series.name.as_str()))
        .execute(conn)?;
    Ok(())
}

pub fn delete_by_id(conn: &mut PgConnection, series_id: i32) -> Result<(), Error> {
    diesel::delete(series::table.find(series_id)).execute(conn)?;
    Ok(())
}
