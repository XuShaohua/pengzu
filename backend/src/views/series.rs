// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use actix_web::{web, HttpResponse};
use shared::books_query::GetBooksQuery;
use shared::general_query::GeneralQuery;

use crate::db::DbPool;
use crate::error::Error;
use crate::models::{books_series, series};

pub async fn add_series(
    pool: web::Data<DbPool>,
    new_series: web::Json<series::NewSeries>,
) -> Result<HttpResponse, Error> {
    web::block(move || {
        let mut conn = pool.get()?;
        series::add_series(&mut conn, &new_series)
    })
    .await??;
    Ok(HttpResponse::Ok().finish())
}

pub async fn get_series_list(
    pool: web::Data<DbPool>,
    query: web::Query<GeneralQuery>,
) -> Result<HttpResponse, Error> {
    let resp_series = web::block(move || {
        let mut conn = pool.get()?;
        series::get_series_list(&mut conn, &query)
    })
    .await??;
    Ok(HttpResponse::Ok().json(resp_series))
}

pub async fn get_series(
    pool: web::Data<DbPool>,
    series_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let resp = web::block(move || {
        let mut conn = pool.get()?;
        series::get_series_by_id(&mut conn, series_id.into_inner())
    })
    .await??;
    Ok(HttpResponse::Ok().json(resp))
}

pub async fn get_books_by_series(
    pool: web::Data<DbPool>,
    series_id: web::Path<i32>,
    query: web::Query<GetBooksQuery>,
) -> Result<HttpResponse, Error> {
    let resp = web::block(move || {
        let mut conn = pool.get()?;
        books_series::get_books_by_series(&mut conn, series_id.into_inner(), &query)
    })
    .await??;
    Ok(HttpResponse::Ok().json(resp))
}

pub async fn delete_series(
    pool: web::Data<DbPool>,
    series_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    web::block(move || {
        let mut conn = pool.get()?;
        let series_id = series_id.into_inner();
        books_series::delete_by_series_id(&mut conn, series_id)?;
        series::delete_by_id(&mut conn, series_id)
    })
    .await??;
    Ok(HttpResponse::Ok().finish())
}
