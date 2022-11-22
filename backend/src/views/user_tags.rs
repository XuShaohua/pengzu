// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use actix_web::{web, HttpResponse};
use shared::books_query::GetBooksQuery;
use shared::recursive_query::RecursiveQuery;

use crate::db::DbPool;
use crate::error::Error;
use crate::models::{books_user_tags, user_tags};

pub async fn add_tag(
    pool: web::Data<DbPool>,
    new_tag: web::Json<user_tags::NewUserTag>,
) -> Result<HttpResponse, Error> {
    web::block(move || {
        let mut conn = pool.get()?;
        user_tags::add_tag(&mut conn, &new_tag)
    })
    .await??;
    Ok(HttpResponse::Ok().finish())
}

pub async fn get_tags(
    pool: web::Data<DbPool>,
    query: web::Query<RecursiveQuery>,
) -> Result<HttpResponse, Error> {
    let tags_resp = web::block(move || {
        let mut conn = pool.get()?;
        user_tags::get_tags(&mut conn, &query)
    })
    .await??;
    Ok(HttpResponse::Ok().json(tags_resp))
}

pub async fn get_tag(
    pool: web::Data<DbPool>,
    tag_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let resp = web::block(move || {
        let mut conn = pool.get()?;
        user_tags::get_tag_by_id(&mut conn, tag_id.into_inner())
    })
    .await??;
    Ok(HttpResponse::Ok().json(resp))
}

pub async fn update_tag(
    pool: web::Data<DbPool>,
    tag_id: web::Path<i32>,
    new_tag: web::Json<user_tags::NewUserTag>,
) -> Result<HttpResponse, Error> {
    web::block(move || {
        let mut conn = pool.get()?;
        user_tags::update_tag(&mut conn, tag_id.into_inner(), &new_tag)
    })
    .await??;
    Ok(HttpResponse::Ok().finish())
}

pub async fn get_books(
    pool: web::Data<DbPool>,
    tag_id: web::Path<i32>,
    query: web::Query<GetBooksQuery>,
) -> Result<HttpResponse, Error> {
    let resp = web::block(move || {
        let mut conn = pool.get()?;
        books_user_tags::get_books(&mut conn, tag_id.into_inner(), &query)
    })
    .await??;
    Ok(HttpResponse::Ok().json(resp))
}
