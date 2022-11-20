// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use actix_files::NamedFile;
use actix_web::{web, HttpResponse};
use shared::files::FileQuery;

use crate::db::DbPool;
use crate::error::{Error, ErrorKind};
use crate::models::files;
use crate::settings;

pub async fn add_file(
    pool: web::Data<DbPool>,
    new_file: web::Json<files::NewFile>,
) -> Result<HttpResponse, Error> {
    web::block(move || {
        let mut conn = pool.get()?;
        files::add_file(&mut conn, &new_file)
    })
    .await??;
    Ok(HttpResponse::Ok().finish())
}

pub async fn get_book_files(
    pool: web::Data<DbPool>,
    book_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let resp_files = web::block(move || {
        let mut conn = pool.get()?;
        files::get_book_files_and_formats(&mut conn, book_id.into_inner())
    })
    .await??;
    Ok(HttpResponse::Ok().json(resp_files))
}

pub async fn get_file_by_path(
    pool: web::Data<DbPool>,
    query: web::Query<FileQuery>,
) -> Result<NamedFile, Error> {
    log::info!("filepath: {:?}", query.path);
    // 1. check auth token

    // 2. check book file exists
    let book_file_path = {
        let book_id = query.book;
        let format_id = query.format;
        web::block(move || {
            let mut conn = pool.get()?;
            files::get_book_file_path(&mut conn, book_id, format_id)
        })
        .await??
    };
    if book_file_path != query.path {
        return Err(Error::from_string(
            ErrorKind::IoError,
            format!("Invalid filepath: {:?}", query),
        ));
    }

    // 3. add download history record

    // 4. return real file path
    let root_dir = settings::get_library_root_dir()?;
    let filepath = root_dir.join(&query.path);
    log::info!("real path: {:?}", filepath);
    if !filepath.starts_with(root_dir) {
        return Err(Error::from_string(
            ErrorKind::IoError,
            format!("Invalid filepath: {:?}", query),
        ));
    }
    let file = NamedFile::open(filepath)?;
    Ok(file.use_last_modified(true))
}
