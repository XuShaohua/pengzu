// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use actix_web::{web, HttpResponse};

use crate::db::DbPool;
use crate::error::Error;
use crate::models::{common_page, series};

pub async fn add_series(
    pool: web::Data<DbPool>,
    new_series: web::Json<series::NewSeries>,
) -> Result<HttpResponse, Error> {
    web::block(move || {
        let conn = pool.get()?;
        series::add_series(&conn, &new_series)
    })
    .await??;
    Ok(HttpResponse::Ok().finish())
}

pub async fn get_series(
    pool: web::Data<DbPool>,
    query: web::Query<common_page::PageQuery>,
) -> Result<HttpResponse, Error> {
    let resp_series = web::block(move || {
        let conn = pool.get()?;
        series::get_series(&conn, &query)
    })
    .await??;
    Ok(HttpResponse::Ok().json(resp_series))
}
