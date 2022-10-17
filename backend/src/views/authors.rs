// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use actix_web::{web, HttpResponse};

use crate::db::DbPool;
use crate::error::Error;
use crate::models::authors;
use crate::models::page::PageQuery;

pub async fn add_author(
    pool: web::Data<DbPool>,
    new_author: web::Json<authors::NewAuthor>,
) -> Result<HttpResponse, Error> {
    web::block(move || {
        let mut conn = pool.get()?;
        authors::add_author(&mut conn, &new_author)
    })
    .await??;
    Ok(HttpResponse::Ok().finish())
}

pub async fn get_authors(
    pool: web::Data<DbPool>,
    query: web::Query<PageQuery>,
) -> Result<HttpResponse, Error> {
    let resp = web::block(move || {
        let mut conn = pool.get()?;
        authors::get_authors(&mut conn, &query)
    })
    .await??;
    Ok(HttpResponse::Ok().json(resp))
}

pub async fn get_author(
    pool: web::Data<DbPool>,
    author_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let resp = web::block(move || {
        let mut conn = pool.get()?;
        authors::get_author_by_id(&mut conn, author_id.into_inner())
    })
    .await??;
    Ok(HttpResponse::Ok().json(resp))
}
