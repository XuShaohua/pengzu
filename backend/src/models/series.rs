// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use chrono::NaiveDateTime;
use diesel::{
    ExpressionMethods, GroupByDsl, Insertable, JoinOnDsl, PgConnection, QueryDsl, Queryable,
    RunQueryDsl,
};
use serde::{Deserialize, Serialize};
use shared_models::page::{Page, PageQuery};

use crate::error::Error;
use crate::schema::series;

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "series"]
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

pub fn add_series(conn: &PgConnection, new_series: &NewSeries) -> Result<(), Error> {
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

pub fn get_series(conn: &PgConnection, query: &PageQuery) -> Result<GetSeriesResp, Error> {
    use crate::schema::books_series_link;

    let page_id = if query.page < 1 { 0 } else { query.page - 1 };
    let each_page = 50;
    let offset = page_id * each_page;

    let list = series::table
        .left_join(books_series_link::table.on(books_series_link::series.eq(series::id)))
        .group_by(series::id)
        .select((
            series::id,
            series::name,
            diesel::dsl::sql::<diesel::sql_types::BigInt>("count(books_series_link.id)"),
        ))
        .limit(each_page)
        .offset(offset)
        .load::<SeriesAndBook>(conn)?;

    let total = series::dsl::series.count().first(conn)?;

    Ok(GetSeriesResp {
        page: Page {
            page_num: page_id + 1,
            each_page,
            total,
        },
        list,
    })
}

pub fn update_series(
    conn: &PgConnection,
    pub_id: i32,
    new_series: &NewSeries,
) -> Result<(), Error> {
    use crate::schema::series::dsl::{name, series};
    diesel::update(series.find(pub_id))
        .set(name.eq(new_series.name.as_str()))
        .execute(conn)?;
    Ok(())
}
