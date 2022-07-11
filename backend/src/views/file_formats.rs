// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use actix_web::{web, HttpResponse};
use shared_models::page::PageQuery;

use crate::db::DbPool;
use crate::error::Error;
use crate::models::file_formats;

pub async fn get_formats(
    pool: web::Data<DbPool>,
    query: web::Query<PageQuery>,
) -> Result<HttpResponse, Error> {
    let resp = web::block(move || {
        let conn = pool.get()?;
        file_formats::get_formats(&conn, &query)
    })
    .await??;
    Ok(HttpResponse::Ok().json(resp))
}
