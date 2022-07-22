// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use chrono::NaiveDateTime;
use gloo_storage::Storage;
use serde::{Deserialize, Serialize};

use crate::components::models::error::FetchError;
use crate::components::models::fetch::fetch_post;

const USER_INFO_STORAGE_KEY: &str = "user-info";

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[repr(u8)]
pub enum UserRole {
    User = 0,
    Admin = 2,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: i32,
    pub name: String,
    pub display_name: String,
    pub email: String,
    pub role: UserRole,
    pub created: NaiveDateTime,
}

// TODO(Shaohua): Replace String with &str.
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

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
