// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use gloo_storage::Storage;

use crate::error::FetchError;
use crate::services::auth::set_token;
use crate::services::fetch::{request_get, request_post};
use crate::types::users::{LoginForm, UserInfo};

const STORAGE_KEY_USER_INFO: &str = "user-info";

pub async fn login(form: &LoginForm) -> Result<UserInfo, FetchError> {
    let url = "/api/login";
    let user_info: UserInfo = request_post(url, form).await?;
    set_token(&user_info.token);
    Ok(user_info)
}

pub async fn fetch_users() -> Result<Vec<UserInfo>, FetchError> {
    request_get("/api/user").await
}

pub fn get_user_info() -> Option<UserInfo> {
    let storage = gloo_storage::LocalStorage::raw();
    match storage.get(STORAGE_KEY_USER_INFO) {
        Ok(Some(text)) => serde_json::from_str(&text).unwrap_or_else(|_| None),
        Ok(None) => None,
        Err(err) => {
            log::error!("Failed to read local storage: {:?}", err);
            None
        }
    }
}
