// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use actix_web::{web, HttpResponse};

use crate::db::DbPool;
use crate::error::Error;
use crate::models::common_page;
use crate::models::file_formats;

pub async fn get_authors(
    pool: web::Data<DbPool>,
    query: web::Query<common_page::PageQuery>,
) -> Result<HttpResponse, Error> {
    let resp = web::block(move || {
        let conn = pool.get()?;
        file_formats::get_authors(&conn, &query)
    })
    .await??;
    Ok(HttpResponse::Ok().json(resp))
}
