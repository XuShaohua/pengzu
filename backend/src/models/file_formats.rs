// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use diesel::{ExpressionMethods, Insertable, JoinOnDsl, PgConnection, QueryDsl, RunQueryDsl};
use serde::Deserialize;
use shared::books::BookAndAuthorsList;
use shared::books_query::GetBooksQuery;
use shared::file_formats::{FileFormat, FileFormatAndBook, FileFormatAndBookList};
use shared::page::{Page, PageQuery, BOOKS_EACH_PAGE, FILE_FORMATS_EACH_PAGE};

use crate::error::Error;
use crate::models::books::{book_list_to_book_authors, Book};
use crate::models::books_query::sort_books_by_column;
use crate::schema::file_formats;

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = file_formats)]
pub struct NewFileFormat {
    pub name: String,
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

pub fn get_formats(
    conn: &mut PgConnection,
    query: &PageQuery,
) -> Result<FileFormatAndBookList, Error> {
    use crate::schema::files;

    let offset = query.backend_page_id() * FILE_FORMATS_EACH_PAGE;

    let list = file_formats::table
        .left_join(files::table.on(files::format.eq(file_formats::id)))
        .group_by(file_formats::id)
        .select((
            file_formats::id,
            file_formats::name,
            diesel::dsl::sql::<diesel::sql_types::BigInt>("count(files.id)"),
        ))
        .limit(FILE_FORMATS_EACH_PAGE)
        .offset(offset)
        .load::<FileFormatAndBook>(conn)?;

    let total = file_formats::table.count().first(conn)?;

    Ok(FileFormatAndBookList {
        page: Page {
            page_num: query.frontend_page_id(),
            each_page: FILE_FORMATS_EACH_PAGE,
            total,
        },
        list,
    })
}

pub fn get_books_by_file_format(
    conn: &mut PgConnection,
    format_id: i32,
    query: &GetBooksQuery,
) -> Result<BookAndAuthorsList, Error> {
    use crate::schema::{books, files};

    let offset = query.backend_page_id() * BOOKS_EACH_PAGE;
    let total = files::table
        .filter(files::format.eq(format_id))
        .count()
        .first::<i64>(conn)?;

    // Get book list based on a subquery.
    let book_list = sort_books_by_column(query.order)
        .filter(
            books::id.eq_any(
                files::table
                    .filter(files::format.eq(format_id))
                    .select(files::book),
            ),
        )
        .limit(BOOKS_EACH_PAGE)
        .offset(offset)
        .load::<Book>(conn)?;

    book_list_to_book_authors(conn, book_list, query, total)
}
