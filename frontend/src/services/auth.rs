// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use gloo_storage::Storage;

const STORAGE_KEY_TOKEN: &str = "token";

pub fn get_token() -> Option<String> {
    let storage = gloo_storage::LocalStorage::raw();
    storage.get(STORAGE_KEY_TOKEN).unwrap()
}

pub fn set_token(token: &str) {
    let storage = gloo_storage::LocalStorage::raw();
    if let Err(err) = storage.set(STORAGE_KEY_TOKEN, token) {
        log::error!("Failed to store token to local storage, err: {:?}", err);
    }
}
