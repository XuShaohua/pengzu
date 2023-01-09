// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use actix_web::{web, HttpRequest, HttpResponse};
use shared::books_query::GetBooksQuery;
use shared::recursive_query::RecursiveQuery;

use crate::db::DbPool;
use crate::error::Error;
use crate::models::user_tags::NewUserTag;
use crate::models::{books_user_tags, user_tags};
use crate::views::auth::get_claims_from_auth;

pub async fn add_tag(
    pool: web::Data<DbPool>,
    new_tag: web::Json<NewUserTag>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let claims = get_claims_from_auth(&req)?;
    let user_id = claims.id();

    web::block(move || {
        let mut conn = pool.get()?;
        let new_tag_with_id = NewUserTag {
            order_index: new_tag.order_index,
            name: new_tag.name.clone(),
            parent: new_tag.parent,
            user_id,
        };
        user_tags::add_tag(&mut conn, &new_tag_with_id)
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
    new_tag: web::Json<NewUserTag>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let claims = get_claims_from_auth(&req)?;
    let user_id = claims.id();

    web::block(move || {
        let mut conn = pool.get()?;
        let new_tag_with_id = NewUserTag {
            order_index: new_tag.order_index,
            name: new_tag.name.clone(),
            parent: new_tag.parent,
            user_id,
        };
        user_tags::update_tag(&mut conn, tag_id.into_inner(), &new_tag_with_id)
    })
    .await??;
    Ok(HttpResponse::Ok().finish())
}

pub async fn get_books_by_user_tag(
    pool: web::Data<DbPool>,
    tag_id: web::Path<i32>,
    query: web::Query<GetBooksQuery>,
) -> Result<HttpResponse, Error> {
    let resp = web::block(move || {
        let mut conn = pool.get()?;
        books_user_tags::get_books_by_user_tag(&mut conn, tag_id.into_inner(), &query)
    })
    .await??;
    Ok(HttpResponse::Ok().json(resp))
}

pub async fn delete_tag(
    pool: web::Data<DbPool>,
    tag_id: web::Path<i32>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let claims = get_claims_from_auth(&req)?;
    let user_id = claims.id();

    web::block(move || {
        // TODO(Shaohua): Delete books-user-tags link first.
        let mut conn = pool.get()?;
        user_tags::delete_by_id(&mut conn, tag_id.into_inner(), user_id)
    })
    .await??;
    Ok(HttpResponse::Ok().finish())
}
