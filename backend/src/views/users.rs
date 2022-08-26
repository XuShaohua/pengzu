// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use actix_web::cookie::Cookie;
use actix_web::{web, HttpResponse};

use crate::db::DbPool;
use crate::error::Error;
use crate::models::users;
use crate::models::users::NewUserReq;
use crate::views::auth::{Claims, UserPermissions, TOKEN_NAME};

pub async fn login(
    pool: web::Data<DbPool>,
    form: web::Json<users::LoginForm>,
) -> Result<HttpResponse, Error> {
    let mut user_info: users::UserInfo = web::block(move || {
        let conn = pool.get()?;
        users::login(&conn, &form)
    })
    .await??;

    let permission = UserPermissions {
        id: user_info.id,
        name: user_info.name.clone(),
        role: user_info.role,
    };
    let claims = Claims::new(&permission);
    let token = claims.encode()?;
    let mut cookie = Cookie::new(TOKEN_NAME, &token);
    cookie.set_path("/");
    cookie.set_expires(claims.exp_offset());
    user_info.token = token.clone();
    let mut resp = HttpResponse::Ok().json(user_info);
    resp.add_cookie(&cookie)?;
    Ok(resp)
}

pub async fn add_user(
    pool: web::Data<DbPool>,
    new_user: web::Json<NewUserReq>,
) -> Result<HttpResponse, Error> {
    let user_info = web::block(move || {
        let conn = pool.get()?;
        users::add_user(&conn, new_user.into_inner())
    })
    .await??;

    Ok(HttpResponse::Ok().json(user_info))
}

pub async fn get_users(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let users_resp = web::block(move || {
        let conn = pool.get()?;
        users::get_users(&conn)
    })
    .await??;
    Ok(HttpResponse::Ok().json(users_resp))
}

pub async fn delete_user(
    pool: web::Data<DbPool>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    web::block(move || {
        let conn = pool.get()?;
        users::delete_user(&conn, user_id.into_inner())
    })
    .await??;
    Ok(HttpResponse::Ok().finish())
}
