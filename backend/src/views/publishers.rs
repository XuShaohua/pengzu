// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use actix_web::{web, HttpResponse};
use shared::books_query::GetBooksQuery;
use shared::general_query::GeneralQuery;

use crate::db::DbPool;
use crate::error::Error;
use crate::models::{books_publishers, publishers};

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
    query: web::Query<GeneralQuery>,
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
    query: web::Query<GetBooksQuery>,
) -> Result<HttpResponse, Error> {
    let resp = web::block(move || {
        let mut conn = pool.get()?;
        books_publishers::get_books_by_publisher(&mut conn, publisher_id.into_inner(), &query)
    })
    .await??;
    Ok(HttpResponse::Ok().json(resp))
}

pub async fn delete_publisher(
    pool: web::Data<DbPool>,
    publisher_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    web::block(move || {
        let mut conn = pool.get()?;
        let publisher_id = publisher_id.into_inner();
        books_publishers::delete_by_publisher_id(&mut conn, publisher_id)?;
        publishers::delete_by_id(&mut conn, publisher_id)
    })
    .await??;
    Ok(HttpResponse::Ok().finish())
}
