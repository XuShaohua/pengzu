// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use actix_web::{web, HttpResponse};

use crate::db::DbPool;
use crate::error::Error;
use crate::models::advanced_search;

pub async fn get_books_by_advanced_search(
    pool: web::Data<DbPool>,
    query: web::Query<advanced_search::AdvancedSearchQuery>,
) -> Result<HttpResponse, Error> {
    let resp = web::block(move || {
        let mut conn = pool.get()?;
        advanced_search::get_books_by_advanced_search(&mut conn, &query)
    })
    .await??;
    Ok(HttpResponse::Ok().json(resp))
}
