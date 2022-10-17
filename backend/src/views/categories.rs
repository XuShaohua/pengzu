// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use actix_web::{web, HttpResponse};

use crate::db::DbPool;
use crate::error::Error;
use crate::models::categories;
use crate::models::categories::GetCategoriesReq;

pub async fn get_categories(
    pool: web::Data<DbPool>,
    query: web::Query<GetCategoriesReq>,
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
