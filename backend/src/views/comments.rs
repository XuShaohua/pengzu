// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use actix_web::{web, HttpResponse};

use crate::db::DbPool;
use crate::error::Error;
use crate::models::comments;

pub async fn add_comment(
    pool: web::Data<DbPool>,
    new_comment: web::Json<comments::NewComment>,
) -> Result<HttpResponse, Error> {
    web::block(move || {
        let conn = pool.get()?;
        comments::add_comment(&conn, &new_comment)
    })
    .await??;
    Ok(HttpResponse::Ok().finish())
}

pub async fn get_comment(
    pool: web::Data<DbPool>,
    book_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let resp_comment = web::block(move || {
        let conn = pool.get()?;
        comments::get_comment(&conn, book_id.into_inner())
    })
    .await??;
    Ok(HttpResponse::Ok().json(resp_comment))
}

pub async fn update_comment(
    pool: web::Data<DbPool>,
    book_id: web::Path<i32>,
    new_comment: web::Json<comments::NewComment>,
) -> Result<HttpResponse, Error> {
    debug_assert_eq!(book_id.into_inner(), new_comment.book);

    web::block(move || {
        let conn = pool.get()?;
        comments::update_comment(&conn, &new_comment)
    })
    .await??;

    Ok(HttpResponse::Ok().finish())
}

pub async fn delete_comment(
    pool: web::Data<DbPool>,
    book_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    web::block(move || {
        let conn = pool.get()?;
        comments::delete_comment(&conn, book_id.into_inner())
    })
    .await??;

    Ok(HttpResponse::Ok().finish())
}
