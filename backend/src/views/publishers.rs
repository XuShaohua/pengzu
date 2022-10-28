// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use actix_web::{web, HttpResponse};

use crate::db::DbPool;
use crate::error::Error;
use crate::models::page::PageQuery;
use crate::models::{books, publishers};

pub async fn add_publisher(
    pool: web::Data<DbPool>,
    new_publisher: web::Json<publishers::NewPublisher>,
) -> Result<HttpResponse, Error> {
    web::block(move || {
        let mut conn = pool.get()?;
        publishers::add_publisher(&mut conn, &new_publisher)
    })
    .await??;
    Ok(HttpResponse::Ok().finish())
}

pub async fn get_publishers(
    pool: web::Data<DbPool>,
    query: web::Query<PageQuery>,
) -> Result<HttpResponse, Error> {
    let resp_publishers = web::block(move || {
        let mut conn = pool.get()?;
        publishers::get_publishers(&mut conn, &query)
    })
    .await??;
    Ok(HttpResponse::Ok().json(resp_publishers))
}

pub async fn get_publisher(
    pool: web::Data<DbPool>,
    publisher_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let resp_publisher = web::block(move || {
        let mut conn = pool.get()?;
        publishers::get_publisher_by_id(&mut conn, publisher_id.into_inner())
    })
    .await??;
    Ok(HttpResponse::Ok().json(resp_publisher))
}

pub async fn get_books_by_publisher(
    pool: web::Data<DbPool>,
    publisher_id: web::Path<i32>,
    query: web::Query<books::GetBooksQuery>,
) -> Result<HttpResponse, Error> {
    let resp = web::block(move || {
        let mut conn = pool.get()?;
        publishers::get_books_by_publisher(&mut conn, publisher_id.into_inner(), &query)
    })
    .await??;
    Ok(HttpResponse::Ok().json(resp))
}
