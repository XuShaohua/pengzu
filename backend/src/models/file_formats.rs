// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use chrono::NaiveDateTime;
use diesel::{
    ExpressionMethods, Insertable, JoinOnDsl, PgConnection, QueryDsl, Queryable, RunQueryDsl,
};
use serde::{Deserialize, Serialize};
use shared::books_query::GetBooksQuery;
use shared::page::{Page, PageQuery};

use crate::error::Error;
use crate::models::books::{get_books_by_ids, GetBooksResp};
use crate::schema::file_formats;

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = file_formats)]
pub struct NewFileFormat {
    pub name: String,
}

#[derive(Debug, Default, Serialize, Queryable)]
pub struct FileFormat {
    pub id: i32,
    pub name: String,
    pub crated: NaiveDateTime,
    pub last_modified: NaiveDateTime,
}

pub fn add_file_format(conn: &mut PgConnection, new_format: &NewFileFormat) -> Result<(), Error> {
    use crate::schema::file_formats::dsl::file_formats;
    diesel::insert_into(file_formats)
        .values(new_format)
        .execute(conn)?;
    Ok(())
}

pub fn get_file_format_by_name(
    conn: &mut PgConnection,
    format_name: &str,
) -> Result<FileFormat, Error> {
    use crate::schema::file_formats::dsl::{file_formats, name};
    file_formats
        .filter(name.eq(format_name))
        .first(conn)
        .map_err(Into::into)
}

pub fn get_file_format_by_id(conn: &mut PgConnection, format_id: i32) -> Result<FileFormat, Error> {
    use crate::schema::file_formats::dsl::file_formats;
    file_formats.find(format_id).first(conn).map_err(Into::into)
}

pub fn get_file_format_by_ids(
    conn: &mut PgConnection,
    ids: &[i32],
) -> Result<Vec<FileFormat>, Error> {
    file_formats::table
        .filter(file_formats::id.eq_any(ids))
        .load(conn)
        .map_err(Into::into)
}

#[derive(Debug, Serialize, Queryable)]
pub struct FileFormatAndBook {
    pub id: i32,
    pub name: String,
    pub count: i64,
}

#[derive(Debug, Serialize)]
pub struct GetFileFormatsResp {
    pub page: Page,
    pub list: Vec<FileFormatAndBook>,
}

pub fn get_formats(
    conn: &mut PgConnection,
    query: &PageQuery,
) -> Result<GetFileFormatsResp, Error> {
    use crate::schema::files;

    let page_id = if query.page < 1 { 0 } else { query.page - 1 };
    let each_page = 100;
    let offset = page_id * each_page;

    let list = file_formats::table
        .left_join(files::table.on(files::format.eq(file_formats::id)))
        .group_by(file_formats::id)
        .select((
            file_formats::id,
            file_formats::name,
            diesel::dsl::sql::<diesel::sql_types::BigInt>("count(files.id)"),
        ))
        .limit(each_page)
        .offset(offset)
        .load::<FileFormatAndBook>(conn)?;

    let total = file_formats::table.count().first(conn)?;

    Ok(GetFileFormatsResp {
        page: Page {
            page_num: page_id + 1,
            each_page,
            total,
        },
        list,
    })
}

pub fn get_books_by_format(
    conn: &mut PgConnection,
    format_id: i32,
    query: &GetBooksQuery,
) -> Result<GetBooksResp, Error> {
    use crate::schema::files;

    let book_ids = files::table
        .filter(files::format.eq(format_id))
        .select(files::book)
        .load::<i32>(conn)?;

    get_books_by_ids(conn, query, &book_ids)
}
