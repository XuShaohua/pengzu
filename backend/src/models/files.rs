// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use diesel::{ExpressionMethods, Insertable, JoinOnDsl, PgConnection, QueryDsl, RunQueryDsl};
use serde::Deserialize;
use shared::file_formats::FileFormat;
use shared::files::{File, FileWithPath};

use crate::error::Error;
use crate::models::books::get_book_path_by_id;
use crate::models::file_formats::get_file_format_by_ids;
use crate::schema::files;

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = files)]
pub struct NewFile {
    pub book: i32,
    pub format: i32,
    pub size: i32,
    pub name: String,
}

pub fn add_file(conn: &mut PgConnection, new_file: &NewFile) -> Result<(), Error> {
    use crate::schema::files::dsl::files;
    diesel::insert_into(files).values(new_file).execute(conn)?;
    Ok(())
}

#[must_use]
fn get_book_format_path(book_path: &str, file_name: &str, format_name: &str) -> String {
    format!(
        "{}/{}.{}",
        book_path,
        file_name,
        format_name.to_ascii_lowercase()
    )
}

pub fn get_book_file_path(
    conn: &mut PgConnection,
    book_id: i32,
    file_id: i32,
) -> Result<String, Error> {
    use crate::schema::{books, file_formats};

    let (book_path, file_name, format_name) = files::table
        .filter(files::id.eq(file_id))
        .filter(files::book.eq(book_id))
        .inner_join(books::table.on(books::id.eq(files::book)))
        .inner_join(file_formats::table.on(file_formats::id.eq(files::format)))
        .select((books::path, files::name, file_formats::name))
        .first::<(String, String, String)>(conn)?;
    let path = get_book_format_path(&book_path, &file_name, &format_name);
    Ok(path)
}

pub fn get_book_files_and_formats(
    conn: &mut PgConnection,
    book_id: i32,
) -> Result<Vec<FileWithPath>, Error> {
    let list = files::table
        .filter(files::book.eq(book_id))
        .load::<File>(conn)?;
    let book_path = get_book_path_by_id(conn, book_id)?;
    let format_id_list: Vec<i32> = list.iter().map(|file| file.format).collect();
    let formats = get_file_format_by_ids(conn, &format_id_list)?;
    let default_format = FileFormat::default();

    Ok(list
        .iter()
        .map(|file| {
            let format = formats
                .iter()
                .find(|format| format.id == file.format)
                .unwrap_or(&default_format);
            let path = get_book_format_path(&book_path, &file.name, &format.name);
            FileWithPath {
                id: file.id,
                book: book_id,
                size: file.size,
                format_id: format.id,
                format_name: format.name.clone(),
                name: file.name.clone(),
                path,
            }
        })
        .collect())
}
