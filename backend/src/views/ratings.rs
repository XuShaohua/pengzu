// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use actix_web::{web, HttpResponse};

use crate::db::DbPool;
use crate::error::Error;
use crate::models::ratings as models;

pub async fn add_rating(
    pool: web::Data<DbPool>,
    new_rating: web::Json<models::NewRating>,
) -> Result<HttpResponse, Error> {
    web::block(move || {
        let conn = pool.get()?;
        models::add_rating(&conn, &new_rating)
    })
    .await??;
    Ok(HttpResponse::Ok().finish())
}

pub async fn get_ratings(
    pool: web::Data<DbPool>,
    book_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let resp_rating = web::block(move || {
        let conn = pool.get()?;
        models::get_rating(&conn, book_id.into_inner())
    })
    .await??;
    Ok(HttpResponse::Ok().json(resp_rating))
}

pub async fn update_rating(
    pool: web::Data<DbPool>,
    book_id: web::Path<i32>,
    new_rating: web::Json<models::NewRating>,
) -> Result<HttpResponse, Error> {
    debug_assert_eq!(book_id.into_inner(), new_rating.book);
    web::block(move || {
        let conn = pool.get()?;
        models::update_rating(&conn, &new_rating)
    })
    .await??;
    Ok(HttpResponse::Ok().finish())
}

pub async fn delete_rating(
    pool: web::Data<DbPool>,
    book_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    web::block(move || {
        let conn = pool.get()?;
        models::delete_rating(&conn, book_id.into_inner())
    })
    .await??;
    Ok(HttpResponse::Ok().finish())
}
