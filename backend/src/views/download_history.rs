// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use actix_web::{web, HttpRequest, HttpResponse};
use shared::books_query::GetBooksQuery;

use crate::db::DbPool;
use crate::error::Error;
use crate::models::download_history;
use crate::views::auth::get_claims_from_cookie;

pub async fn get_books(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    query: web::Query<GetBooksQuery>,
) -> Result<HttpResponse, Error> {
    let claims = get_claims_from_cookie(&req)?;
    let user_id = claims.id();

    let resp = web::block(move || {
        let mut conn = pool.get()?;
        download_history::get_books(&mut conn, user_id, &query)
    })
    .await??;
    Ok(HttpResponse::Ok().json(resp))
}
