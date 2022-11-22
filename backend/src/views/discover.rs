// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use actix_web::{web, HttpResponse};

use crate::db::DbPool;
use crate::error::Error;
use crate::models::discover;

pub async fn get_books(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let resp = web::block(move || {
        let mut conn = pool.get()?;
        discover::get_books(&mut conn)
    })
    .await??;
    Ok(HttpResponse::Ok().json(resp))
}
