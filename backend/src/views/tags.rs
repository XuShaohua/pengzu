// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use actix_web::{web, HttpResponse};
use shared::books_query::GetBooksQuery;
use shared::recursive_query::RecursiveQuery;
use shared::tags::SearchTagQuery;

use crate::db::DbPool;
use crate::error::Error;
use crate::models::{books_tags, tags};

pub async fn add_tag(
    pool: web::Data<DbPool>,
    new_tag: web::Json<tags::NewTag>,
) -> Result<HttpResponse, Error> {
    let tag_resp = web::block(move || {
        let mut conn = pool.get()?;
        tags::add_tag(&mut conn, &new_tag)
    })
    .await??;
    Ok(HttpResponse::Ok().json(tag_resp))
}

pub async fn get_tags(
    pool: web::Data<DbPool>,
    query: web::Query<RecursiveQuery>,
) -> Result<HttpResponse, Error> {
    let tags_resp = web::block(move || {
        let mut conn = pool.get()?;
        tags::get_tags(&mut conn, &query)
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
        tags::get_tag_by_id(&mut conn, tag_id.into_inner())
    })
    .await??;
    Ok(HttpResponse::Ok().json(resp))
}

pub async fn update_tag(
    pool: web::Data<DbPool>,
    tag_id: web::Path<i32>,
    new_tag: web::Json<tags::NewTag>,
) -> Result<HttpResponse, Error> {
    web::block(move || {
        let mut conn = pool.get()?;
        tags::update_tag(&mut conn, tag_id.into_inner(), &new_tag)
    })
    .await??;
    Ok(HttpResponse::Ok().finish())
}

pub async fn delete_tag(
    pool: web::Data<DbPool>,
    tag_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    web::block(move || {
        let mut conn = pool.get()?;
        let tag_id = tag_id.into_inner();
        books_tags::delete_by_tag_id(&mut conn, tag_id)?;
        tags::delete_by_id(&mut conn, tag_id)
    })
    .await??;
    Ok(HttpResponse::Ok().finish())
}

pub async fn get_books_by_tag(
    pool: web::Data<DbPool>,
    tag_id: web::Path<i32>,
    query: web::Query<GetBooksQuery>,
) -> Result<HttpResponse, Error> {
    let resp = web::block(move || {
        let mut conn = pool.get()?;
        books_tags::get_books_by_tag(&mut conn, tag_id.into_inner(), &query)
    })
    .await??;
    Ok(HttpResponse::Ok().json(resp))
}

pub async fn add_book(
    pool: web::Data<DbPool>,
    tag_id: web::Path<i32>,
    book_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    web::block(move || {
        let mut conn = pool.get()?;
        let book_tag = books_tags::NewBookTag {
            book: book_id.into_inner(),
            tag: tag_id.into_inner(),
        };
        books_tags::add_book_tag(&mut conn, &book_tag)
    })
    .await??;
    Ok(HttpResponse::Ok().finish())
}

pub async fn delete_book(
    pool: web::Data<DbPool>,
    tag_id: web::Path<i32>,
    book_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    web::block(move || {
        let mut conn = pool.get()?;
        books_tags::delete_book(&mut conn, tag_id.into_inner(), book_id.into_inner())
    })
    .await??;
    Ok(HttpResponse::Ok().finish())
}

pub async fn cleanup_unused(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    web::block(move || {
        let mut conn = pool.get()?;
        books_tags::cleanup_unused(&mut conn)
    })
    .await??;
    Ok(HttpResponse::Ok().finish())
}

pub async fn search_tags(
    pool: web::Data<DbPool>,
    query: web::Json<SearchTagQuery>,
) -> Result<HttpResponse, Error> {
    let list = web::block(move || {
        let mut conn = pool.get()?;
        tags::search(&mut conn, &query)
    })
    .await??;
    Ok(HttpResponse::Ok().json(list))
}
