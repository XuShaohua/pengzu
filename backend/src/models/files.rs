// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::models::books::get_book_path_by_id;
use crate::models::file_formats::{get_file_format_by_ids, FileFormat};
use crate::schema::files;

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = files)]
pub struct NewFile {
    pub book: i32,
    pub format: i32,
    pub size: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Queryable)]
pub struct File {
    pub id: i32,
    pub book: i32,
    pub format: i32,
    pub size: i32,
    pub name: String,
    pub created: NaiveDateTime,
    pub last_modified: NaiveDateTime,
}

#[derive(Debug, Serialize, Queryable)]
pub struct FileWithPath {
    pub id: i32,
    pub book: i32,
    pub size: i32,
    pub format_id: i32,
    pub format_name: String,
    pub name: String,
    pub path: String,
}

pub fn add_file(conn: &mut PgConnection, new_file: &NewFile) -> Result<(), Error> {
    use crate::schema::files::dsl::files;
    diesel::insert_into(files).values(new_file).execute(conn)?;
    Ok(())
}

pub fn get_book_files_and_formats(
    conn: &mut PgConnection,
    book_id: i32,
) -> Result<Vec<FileWithPath>, Error> {
    let list = files::table
        .filter(files::book.eq(book_id))
        .load::<File>(conn)?;
    let path = get_book_path_by_id(conn, book_id)?;
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
            let path = format!(
                "{}/{}.{}",
                path,
                file.name,
                format.name.to_ascii_lowercase()
            );
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
