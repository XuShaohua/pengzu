// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use actix_web::{web, HttpResponse};

use crate::db::DbPool;
use crate::error::Error;
use crate::models::{authors as models, common_page};

pub async fn add_author(
    pool: web::Data<DbPool>,
    new_author: web::Json<models::NewAuthor>,
) -> Result<HttpResponse, Error> {
    web::block(move || {
        let conn = pool.get()?;
        models::add_author(&conn, &new_author)
    })
    .await??;
    Ok(HttpResponse::Ok().finish())
}

pub async fn get_authors(
    pool: web::Data<DbPool>,
    query: web::Query<common_page::PageQuery>,
) -> Result<HttpResponse, Error> {
    let resp = web::block(move || {
        let conn = pool.get()?;
        models::get_authors(&conn, &query)
    })
    .await??;
    Ok(HttpResponse::Ok().json(resp))
}
