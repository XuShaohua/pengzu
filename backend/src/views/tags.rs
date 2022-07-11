// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use actix_web::{web, HttpResponse};
use shared_models::page::PageQuery;

use crate::db::DbPool;
use crate::error::Error;
use crate::models::tags;

pub async fn add_tag(
    pool: web::Data<DbPool>,
    new_tag: web::Json<tags::NewTag>,
) -> Result<HttpResponse, Error> {
    web::block(move || {
        let conn = pool.get()?;
        tags::add_tag(&conn, &new_tag)
    })
    .await??;
    Ok(HttpResponse::Ok().finish())
}

pub async fn get_tags(
    pool: web::Data<DbPool>,
    query: web::Query<PageQuery>,
) -> Result<HttpResponse, Error> {
    let resp_tags = web::block(move || {
        let conn = pool.get()?;
        tags::get_tags(&conn, &query)
    })
    .await??;
    Ok(HttpResponse::Ok().json(resp_tags))
}

pub async fn update_tag(
    pool: web::Data<DbPool>,
    tag_id: web::Path<i32>,
    new_tag: web::Json<tags::NewTag>,
) -> Result<HttpResponse, Error> {
    web::block(move || {
        let conn = pool.get()?;
        tags::update_tag(&conn, tag_id.into_inner(), &new_tag)
    })
    .await??;
    Ok(HttpResponse::Ok().finish())
}
