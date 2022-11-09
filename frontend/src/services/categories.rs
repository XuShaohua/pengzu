// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared::categories::{Category, CategoryAndBookList};

use crate::error::FetchError;
use crate::services::fetch::request_get;

/// Returns category list.
///
/// # Errors
/// Returns error if server fails.
pub async fn fetch_categories(parent_category_id: i32) -> Result<CategoryAndBookList, FetchError> {
    let url = format!("/api/category?parent={}", parent_category_id);
    request_get(&url).await
}

/// Returns category info.
///
/// # Errors
/// Returns error if server fails.
pub async fn fetch_category(category_id: i32) -> Result<Category, FetchError> {
    let url = format!("/api/category/{}", category_id);
    request_get(&url).await
}
