// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use actix_web::{web, Error, HttpResponse};

use crate::db::DbPool;
use crate::models::comments as models;

pub fn add_comment(
    pool: web::Data<DbPool>,
    form: web::Json<models::Comment>,
) -> Result<HttpResponse, Error> {
    let comment = web::block(move || {
        let conn = pool.get()?;
        models::add_comment(&conn, &form)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(comment))
}

pub fn get_comments() {
    todo!();
}

pub fn update_comment() {
    todo!();
}

pub fn delete_comment() {
    todo!();
}
