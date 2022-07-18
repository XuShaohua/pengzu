// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use crate::error::Error;
use crate::models::users::UserRole;
use crate::views::auth::{Claims, UserPermissions, TOKEN_NAME};
use actix_web::cookie::Cookie;
use actix_web::{web, HttpResponse};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

pub async fn login(form: web::Form<LoginForm>) -> Result<HttpResponse, Error> {
    let mut resp = HttpResponse::Ok().body("Ok");
    let permission = UserPermissions {
        id: 42,
        name: form.username.to_string(),
        role: UserRole::Admin,
    };
    let claims = Claims::new(&permission);
    let token = claims.encode()?;
    let cookie = Cookie::new(TOKEN_NAME, &token);
    resp.add_cookie(&cookie)?;
    Ok(resp)
}
