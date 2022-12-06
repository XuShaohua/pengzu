// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use actix_web::{web, HttpResponse};
use shared::books_query::GetBooksQuery;

use crate::db::DbPool;
use crate::error::Error;
use crate::models::{books, books_meta};

pub async fn add_book(
    pool: web::Data<DbPool>,
    new_book: web::Json<books::NewBook>,
) -> Result<HttpResponse, Error> {
    // TODO(Shaohua): Check user role.
    web::block(move || {
        let mut conn = pool.get()?;
        books::add_book(&mut conn, &new_book)
    })
    .await??;
    Ok(HttpResponse::Ok().finish())
}

pub async fn get_books(
    pool: web::Data<DbPool>,
    query: web::Query<GetBooksQuery>,
) -> Result<HttpResponse, Error> {
    log::info!("query: {:?}", query);
    let resp = web::block(move || {
        let mut conn = pool.get()?;
        books::get_books(&mut conn, &query)
    })
    .await??;

    Ok(HttpResponse::Ok().json(resp))
}

pub async fn get_book_detail(
    pool: web::Data<DbPool>,
    book_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let resp_book = web::block(move || {
        let mut conn = pool.get()?;
        books_meta::get_book_metadata(&mut conn, book_id.into_inner())
    })
    .await??;
    Ok(HttpResponse::Ok().json(resp_book))
}

pub async fn update_book(
    _pool: web::Data<DbPool>,
    _book_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    todo!()
}
