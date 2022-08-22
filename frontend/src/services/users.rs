// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use gloo_storage::Storage;

use crate::services::fetch::fetch_post;
use crate::types::error::FetchError;
use crate::types::users::{LoginForm, UserInfo};

const USER_INFO_STORAGE_KEY: &str = "user-info";

pub async fn login(form: &LoginForm) -> Result<UserInfo, FetchError> {
    let url = "/api/login";
    let body = serde_json::to_string(form)?;
    let text = fetch_post(url, &body).await?;
    let storage = gloo_storage::LocalStorage::raw();
    if let Err(err) = storage.set(USER_INFO_STORAGE_KEY, &text) {
        log::error!("Failed to store user info to local storage, err: {:?}", err);
    }
    let obj: UserInfo = serde_json::from_str(&text)?;
    Ok(obj)
}

pub fn get_user_info() -> Option<UserInfo> {
    let storage = gloo_storage::LocalStorage::raw();
    match storage.get(USER_INFO_STORAGE_KEY) {
        Ok(Some(text)) => serde_json::from_str(&text).unwrap_or_else(|_| None),
        Ok(None) => None,
        Err(err) => {
            log::error!("Failed to read local storage: {:?}", err);
            None
        }
    }
}
