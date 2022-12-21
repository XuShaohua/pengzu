// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use gloo_storage::Storage;
use shared::users::UserInfo;

const STORAGE_KEY_TOKEN: &str = "auth.token";
const STORAGE_KEY_USER_INFO: &str = "auth.user-info";

#[must_use]
pub fn get_token() -> Option<String> {
    let storage = gloo_storage::LocalStorage::raw();
    match storage.get(STORAGE_KEY_TOKEN) {
        Ok(value) => value,
        Err(err) => {
            log::warn!("Failed to get token {STORAGE_KEY_TOKEN}, err: {err:?}");
            None
        }
    }
}

pub fn set_token(token: Option<&str>) {
    let storage = gloo_storage::LocalStorage::raw();

    token.map_or_else(
        || {
            if let Err(err) = storage.delete(STORAGE_KEY_TOKEN) {
                log::error!("Failed to delete token from local storage, err: {err:?}");
            }
        },
        |token| {
            debug_assert!(!token.is_empty());
            if let Err(err) = storage.set(STORAGE_KEY_TOKEN, token) {
                log::error!("Failed to store token to local storage, err: {err:?}");
            }
        },
    );
}

#[must_use]
pub fn load_user_info() -> Option<UserInfo> {
    let storage = gloo_storage::LocalStorage::raw();
    if let Ok(Some(value)) = storage.get(STORAGE_KEY_USER_INFO) {
        serde_json::from_str(&value).ok()
    } else {
        log::info!("Failed to get user info {STORAGE_KEY_USER_INFO}");
        None
    }
}

pub fn save_user_info(user_info: &UserInfo) {
    let storage = gloo_storage::LocalStorage::raw();
    serde_json::to_string(user_info).map_or_else(
        |err| log::warn!("Failed to series user info to string, err: {err:?}"),
        |value| {
            if let Err(err) = storage.set(STORAGE_KEY_USER_INFO, &value) {
                log::warn!("Failed to save user info to storage, err: {err:?}");
            }
        },
    );
}
