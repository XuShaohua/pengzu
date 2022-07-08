// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use actix_web::{web, HttpResponse};

use crate::db::DbPool;
use crate::error::Error;
use crate::models::authors as models;
use crate::models::books;

pub async fn add_author(
    pool: web::Data<DbPool>,
    new_author: web::Json<models::NewAuthor>,
) -> Result<HttpResponse, Error> {
    web::block(move || {
        let conn = pool.get()?;
        models::add_author(&conn, &new_author)
    })
    .await??;
    Ok(HttpResponse::Ok().finish())
}

pub async fn get_authors(
    pool: web::Data<DbPool>,
    query: web::Query<models::GetAuthorsQuery>,
) -> Result<HttpResponse, Error> {
    let resp = web::block(move || {
        let conn = pool.get()?;
        models::get_authors(&conn, &query)
    })
    .await??;
    Ok(HttpResponse::Ok().json(resp))
}

pub async fn get_books_by_author(
    pool: web::Data<DbPool>,
    author_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let resp = web::block(move || {
        let conn = pool.get()?;
        books::get_books_by_author(&conn, author_id.into_inner())
    })
    .await??;
    Ok(HttpResponse::Ok().json(resp))
}
