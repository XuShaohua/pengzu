// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use actix_web::{web, HttpResponse};
use shared::books_query::GetBooksQuery;
use shared::recursive_query::RecursiveQuery;

use crate::db::DbPool;
use crate::error::Error;
use crate::models::{books_categories, categories};

pub async fn get_categories(
    pool: web::Data<DbPool>,
    query: web::Query<RecursiveQuery>,
) -> Result<HttpResponse, Error> {
    let categories_resp = web::block(move || {
        let mut conn = pool.get()?;
        categories::get_categories(&mut conn, &query)
    })
    .await??;
    Ok(HttpResponse::Ok().json(categories_resp))
}

pub async fn get_category(
    pool: web::Data<DbPool>,
    category_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let category_resp = web::block(move || {
        let mut conn = pool.get()?;
        categories::get_category_by_id(&mut conn, category_id.into_inner())
    })
    .await??;
    Ok(HttpResponse::Ok().json(category_resp))
}

pub async fn get_books_by_category(
    pool: web::Data<DbPool>,
    category_id: web::Path<i32>,
    query: web::Query<GetBooksQuery>,
) -> Result<HttpResponse, Error> {
    let resp = web::block(move || {
        let mut conn = pool.get()?;
        books_categories::get_books_by_category(&mut conn, category_id.into_inner(), &query)
    })
    .await??;
    Ok(HttpResponse::Ok().json(resp))
}

pub async fn add_book(
    pool: web::Data<DbPool>,
    category_id: web::Path<i32>,
    book_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    web::block(move || {
        let mut conn = pool.get()?;
        books_categories::add_book(&mut conn, category_id.into_inner(), book_id.into_inner())
    })
    .await??;
    Ok(HttpResponse::Ok().finish())
}

pub async fn delete_book(
    pool: web::Data<DbPool>,
    category_id: web::Path<i32>,
    book_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    web::block(move || {
        let mut conn = pool.get()?;
        books_categories::delete_book(&mut conn, category_id.into_inner(), book_id.into_inner())
    })
    .await??;
    Ok(HttpResponse::Ok().finish())
}
