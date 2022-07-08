// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use actix_web::{web, HttpResponse};

use crate::db::DbPool;
use crate::error::Error;
use crate::models::files;

pub async fn add_file(
    pool: web::Data<DbPool>,
    new_file: web::Json<files::NewFile>,
) -> Result<HttpResponse, Error> {
    web::block(move || {
        let conn = pool.get()?;
        files::add_file(&conn, &new_file)
    })
    .await??;
    Ok(HttpResponse::Ok().finish())
}

pub async fn get_book_files(
    pool: web::Data<DbPool>,
    book_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let resp_files = web::block(move || {
        let conn = pool.get()?;
        files::get_book_files(&conn, book_id.into_inner())
    })
    .await??;
    Ok(HttpResponse::Ok().json(resp_files))
}
