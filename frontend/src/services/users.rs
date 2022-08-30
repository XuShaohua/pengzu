// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use crate::error::FetchError;
use crate::services::auth::set_token;
use crate::services::fetch::{request_get, request_post};
use crate::types::users::{LoginForm, UserInfo};

pub async fn login(form: &LoginForm) -> Result<UserInfo, FetchError> {
    let url = "/api/login";
    let user_info: UserInfo = request_post(url, form).await?;
    set_token(Some(&user_info.token));
    Ok(user_info)
}

pub async fn fetch_users() -> Result<Vec<UserInfo>, FetchError> {
    request_get("/api/user").await
}

pub async fn get_user_info() -> Result<UserInfo, FetchError> {
    let url = "/api/user/info";
    request_get(url).await
}
