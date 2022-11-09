// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use chrono::NaiveDateTime;
use diesel::{
    ExpressionMethods, Insertable, JoinOnDsl, PgConnection, QueryDsl, Queryable, RunQueryDsl,
};
use serde::{Deserialize, Serialize};
use shared::page::{Page, SERIES_EACH_PAGE};

use crate::error::Error;
use crate::models::books::{get_books_by_ids, GetBooksResp};
use crate::models::books_query::GetBooksQuery;
use crate::models::general_query::GeneralQuery;
use crate::schema::series;

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = series)]
pub struct NewSeries {
    pub name: String,
}

#[derive(Debug, Serialize, Queryable)]
pub struct Series {
    pub id: i32,
    pub name: String,
    pub crated: NaiveDateTime,
    pub last_modified: NaiveDateTime,
}

pub fn add_series(conn: &mut PgConnection, new_series: &NewSeries) -> Result<(), Error> {
    use crate::schema::series::dsl::series;
    diesel::insert_into(series)
        .values(new_series)
        .execute(conn)?;
    Ok(())
}

#[derive(Debug, Serialize, Queryable)]
pub struct SeriesAndBook {
    pub id: i32,
    pub name: String,
    pub count: i64,
}

#[derive(Debug, Serialize)]
pub struct GetSeriesResp {
    pub page: Page,
    pub list: Vec<SeriesAndBook>,
}

pub fn get_series_list(
    conn: &mut PgConnection,
    query: &GeneralQuery,
) -> Result<GetSeriesResp, Error> {
    use crate::schema::books_series_link;

    let page_id = if query.page < 1 { 0 } else { query.page - 1 };
    let offset = page_id * SERIES_EACH_PAGE;

    // TODO(Shaohua): Support query order

    let list = series::table
        .left_join(books_series_link::table.on(books_series_link::series.eq(series::id)))
        .group_by(series::id)
        .select((
            series::id,
            series::name,
            diesel::dsl::sql::<diesel::sql_types::BigInt>("count(books_series_link.id)"),
        ))
        .limit(SERIES_EACH_PAGE)
        .offset(offset)
        .load::<SeriesAndBook>(conn)?;

    let total = series::dsl::series.count().first(conn)?;

    Ok(GetSeriesResp {
        page: Page {
            page_num: page_id + 1,
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
    pub_id: i32,
    new_series: &NewSeries,
) -> Result<(), Error> {
    use crate::schema::series::dsl::{name, series};
    diesel::update(series.find(pub_id))
        .set(name.eq(new_series.name.as_str()))
        .execute(conn)?;
    Ok(())
}

pub fn get_books_by_series(
    conn: &mut PgConnection,
    series_id: i32,
    query: &GetBooksQuery,
) -> Result<GetBooksResp, Error> {
    use crate::schema::books_series_link;

    let book_ids = books_series_link::table
        .filter(books_series_link::series.eq(series_id))
        .select(books_series_link::book)
        .load::<i32>(conn)?;

    get_books_by_ids(conn, query, &book_ids)
}
